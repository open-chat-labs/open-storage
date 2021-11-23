use crate::guards::caller_is_known_user;
use crate::model::blobs::{PutChunkArgs, PutChunkResult};
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::upload_chunk::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_known_user")]
#[trace]
fn upload_chunk(args: Args) -> Response {
    RUNTIME_STATE.with(|state| upload_chunk_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn upload_chunk_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    let now = runtime_state.env.now();

    let put_chunk_args = PutChunkArgs::new(caller, now, args);

    match runtime_state.data.blobs.put_chunk(put_chunk_args) {
        PutChunkResult::Success => Success,
        PutChunkResult::Complete => Success,
        PutChunkResult::BlobAlreadyExists => BlobAlreadyExists,
        PutChunkResult::ChunkAlreadyExists => ChunkAlreadyExists,
        PutChunkResult::HashMismatch => HashMismatch,
    }
}
