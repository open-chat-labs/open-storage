use crate::guards::caller_is_bucket;
use crate::BlobRecord;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::c2c_add_blob_reference::*;

#[update(guard = "caller_is_bucket")]
#[trace]
fn c2c_add_blob_reference(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_add_blob_reference_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_add_blob_reference_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    if let Some(user) = runtime_state.data.users.get_mut(&args.user_id) {
        if user.bytes_used + args.blob_size > user.byte_limit {
            return Response::AllowanceReached;
        } else {
            user.bytes_used += args.blob_size;
        }
    } else {
        return Response::UserNotFound;
    }

    let bucket = runtime_state.env.caller();

    runtime_state.data.blobs.entry(args.blob_hash).or_insert_with(|| BlobRecord {
        bucket,
        size: args.blob_size,
    });

    Response::Success
}
