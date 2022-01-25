use crate::{calc_chunk_count, read_state, RuntimeState, LOG_MESSAGES};
use candid::Func;
use canister_logger::LogMessagesContainer;
use http_request::{
    encode_logs, extract_route, get_metrics, HeaderField, HttpRequest, HttpResponse, Route, StreamingCallbackHttpResponse,
    StreamingStrategy, Token,
};
use ic_cdk_macros::query;
use num_traits::cast::ToPrimitive;
use serde_bytes::ByteBuf;
use std::borrow::Cow;
use std::cmp::min;
use types::{BlobId, TimestampMillis};

const BLOB_RESPONSE_CHUNK_SIZE_BYTES: u32 = 1 << 19; // 1/2 MB
const CACHE_HEADER_VALUE: &str = "public, max-age=100000000, immutable";

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_logs_impl(since: Option<TimestampMillis>, messages_container: &LogMessagesContainer) -> HttpResponse {
        encode_logs(messages_container.get(since.unwrap_or(0)))
    }

    fn get_metrics_impl(runtime_state: &RuntimeState) -> HttpResponse {
        get_metrics(&runtime_state.metrics())
    }

    match extract_route(&request.url) {
        Route::Blob(blob_id) => read_state(|state| start_streaming_blob(blob_id, state)),
        Route::Logs(since) => LOG_MESSAGES.with(|l| get_logs_impl(since, &l.borrow().logs)),
        Route::Traces(since) => LOG_MESSAGES.with(|l| get_logs_impl(since, &l.borrow().traces)),
        Route::Metrics => read_state(get_metrics_impl),
        _ => HttpResponse::not_found(),
    }
}

#[query]
fn http_request_streaming_callback(token: Token) -> StreamingCallbackHttpResponse {
    read_state(|state| continue_streaming_blob(token, state))
}

fn start_streaming_blob(blob_id: BlobId, runtime_state: &RuntimeState) -> HttpResponse {
    if let Some(blob_reference) = runtime_state.data.blobs.blob_reference(&blob_id) {
        if let Some(blob_bytes) = runtime_state.data.blobs.blob_bytes(&blob_reference.hash) {
            let canister_id = runtime_state.env.canister_id();

            let (chunk_bytes, stream_next_chunk) = chunk_bytes(blob_bytes, 0);

            let streaming_strategy = if stream_next_chunk {
                Some(StreamingStrategy::Callback {
                    callback: Func {
                        principal: canister_id,
                        method: "http_request_streaming_callback".to_string(),
                    },
                    token: build_token(blob_id, 1),
                })
            } else {
                None
            };

            return HttpResponse {
                status_code: 200,
                headers: vec![
                    HeaderField("Content-Type".to_string(), blob_reference.mime_type.clone()),
                    HeaderField("Cache-Control".to_string(), CACHE_HEADER_VALUE.to_string()),
                ],
                body: Cow::Owned(chunk_bytes),
                streaming_strategy,
            };
        }
    }

    HttpResponse::not_found()
}

fn continue_streaming_blob(token: Token, runtime_state: &RuntimeState) -> StreamingCallbackHttpResponse {
    if let Route::Blob(blob_id) = extract_route(&token.key) {
        let chunk_index = token.index.0.to_u32().unwrap();
        let blobs = &runtime_state.data.blobs;

        if let Some(bytes) = blobs.blob_reference(&blob_id).map(|r| blobs.blob_bytes(&r.hash)).flatten() {
            let (chunk_bytes, stream_next_chunk) = chunk_bytes(bytes, chunk_index);

            let token = if stream_next_chunk { Some(build_token(blob_id, chunk_index + 1)) } else { None };
            return StreamingCallbackHttpResponse {
                body: chunk_bytes,
                token,
            };
        }
    }

    StreamingCallbackHttpResponse {
        body: ByteBuf::new(),
        token: None,
    }
}

fn chunk_bytes(blob_bytes: &ByteBuf, chunk_index: u32) -> (ByteBuf, bool) {
    let total_size = blob_bytes.len();
    let total_chunks = calc_chunk_count(BLOB_RESPONSE_CHUNK_SIZE_BYTES, total_size as u64);
    let last_chunk_index = total_chunks - 1;
    let stream_next_chunk = chunk_index < last_chunk_index;

    if chunk_index > last_chunk_index {
        panic!("Invalid request");
    }

    let start = (BLOB_RESPONSE_CHUNK_SIZE_BYTES as usize) * (chunk_index as usize);
    let end = min(start + (BLOB_RESPONSE_CHUNK_SIZE_BYTES as usize), total_size);

    (ByteBuf::from(&blob_bytes.as_slice()[start..end]), stream_next_chunk)
}

fn build_token(blob_id: u128, index: u32) -> Token {
    Token {
        key: format!("blobs/{}", blob_id),
        content_encoding: String::default(),
        index: index.into(),
        sha256: None,
    }
}
