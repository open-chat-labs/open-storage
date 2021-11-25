use crate::guards::caller_is_index_canister;
use crate::model::blobs::RemoveBlobReferenceResult;
use crate::model::index_sync_state::EventToSync;
use crate::{RuntimeState, MAX_EVENTS_TO_SYNC_PER_BATCH, RUNTIME_STATE};
use bucket_canister::c2c_sync_index::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use types::BlobReferenceRemoved;

#[update(guard = "caller_is_index_canister")]
#[trace]
fn c2c_sync_index(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_sync_index_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_sync_index_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    for user_id in args.users_added {
        runtime_state.data.users.add(user_id);
    }

    let mut blob_references_removed: Vec<BlobReferenceRemoved> = Vec::new();

    for user_id in args.users_removed {
        if let Some(user) = runtime_state.data.users.remove(user_id) {
            for blob_id in user.blobs_uploaded() {
                if let RemoveBlobReferenceResult::Success(b) = runtime_state.data.blobs.remove_blob_reference(user_id, blob_id)
                {
                    blob_references_removed.push(b)
                }
            }
        }
    }

    for accessor_id in args.accessors_removed {
        blob_references_removed.extend(runtime_state.data.blobs.remove_accessor(&accessor_id));
    }

    if blob_references_removed.len() > MAX_EVENTS_TO_SYNC_PER_BATCH {
        // If there are too many events to sync in a single batch, queue the excess events to be
        // synced later via heartbeat
        let excess = blob_references_removed.split_off(MAX_EVENTS_TO_SYNC_PER_BATCH);

        for removed in excess {
            runtime_state
                .data
                .index_sync_state
                .enqueue(EventToSync::BlobReferenceRemoved(removed));
        }
    }

    Success(SuccessResult { blob_references_removed })
}
