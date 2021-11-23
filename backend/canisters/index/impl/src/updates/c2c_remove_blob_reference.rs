use crate::guards::caller_is_bucket;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::c2c_remove_blob_reference::*;

#[update(guard = "caller_is_bucket")]
#[trace]
fn c2c_remove_blob_reference(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_remove_blob_reference_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_remove_blob_reference_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    if let Some(user) = runtime_state.data.users.get_mut(&args.user_id) {
        user.bytes_used -= args.blob_size;
    } else {
        return Response::UserNotFound;
    }

    if let Some(blob_hash) = args.blob_hash {
        runtime_state.data.blobs.remove_entry(&blob_hash);
    }

    Response::Success
}
