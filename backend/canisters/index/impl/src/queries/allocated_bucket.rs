use crate::{read_state, RuntimeState, DEFAULT_CHUNK_SIZE_BYTES};
use canister_api_macros::trace;
use ic_cdk_macros::query;
use index_canister::allocated_bucket::{Args as ArgsV1, Response as ResponseV1};
use index_canister::allocated_bucket_v2::{Response::*, *};

#[query]
#[trace]
fn allocated_bucket(args: ArgsV1) -> ResponseV1 {
    read_state(|state| allocated_bucket_impl(args.into(), state)).into()
}

#[query]
#[trace]
fn allocated_bucket_v2(args: Args) -> Response {
    read_state(|state| allocated_bucket_impl(args, state))
}

fn allocated_bucket_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let user_id = runtime_state.env.caller();
    if let Some(user) = runtime_state.data.users.get(&user_id) {
        let byte_limit = user.byte_limit;
        let bytes_used = user.bytes_used;
        let bytes_used_after_upload = if runtime_state.data.blobs.has_user_uploaded_blob(&user_id, &args.file_hash) {
            bytes_used
        } else {
            bytes_used
                .checked_add(args.file_size)
                .unwrap_or_else(|| panic!("'bytes_used' overflowed for {}", user_id))
        };

        if bytes_used_after_upload > byte_limit {
            return AllowanceExceeded(AllowanceExceededResult {
                byte_limit,
                bytes_used,
                bytes_used_after_upload,
            });
        }

        let bucket = runtime_state
            .data
            .blobs
            .bucket(&args.file_hash)
            .or_else(|| runtime_state.data.buckets.allocate(args.file_hash));

        if let Some(canister_id) = bucket {
            Success(SuccessResult {
                canister_id,
                chunk_size: DEFAULT_CHUNK_SIZE_BYTES,
                byte_limit,
                bytes_used,
                bytes_used_after_upload,
            })
        } else {
            BucketUnavailable
        }
    } else {
        UserNotFound
    }
}
