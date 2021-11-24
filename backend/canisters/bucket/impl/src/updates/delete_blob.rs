use crate::guards::caller_is_known_user;
use crate::model::blobs::RemoveBlobReferenceResult;
use crate::model::index_sync_queue::EventToSync;
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::delete_blob::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_known_user")]
#[trace]
fn delete_blob(args: Args) -> Response {
    RUNTIME_STATE.with(|state| delete_blob_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn delete_blob_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();

    match runtime_state.data.blobs.remove_blob_reference(caller, args.blob_id) {
        RemoveBlobReferenceResult::Success(b) => {
            runtime_state.data.index_sync_queue.push(EventToSync::BlobReferenceRemoved(b));
            Success
        }
        RemoveBlobReferenceResult::NotAuthorized => NotAuthorized,
        RemoveBlobReferenceResult::NotFound => NotFound,
    }
}
