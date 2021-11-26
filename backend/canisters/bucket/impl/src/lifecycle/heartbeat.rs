use crate::model::users::BlobStatusInternal;
use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::heartbeat;
use index_canister::c2c_sync_bucket::{Args, Response, SuccessResult};
use types::CanisterId;

#[heartbeat]
fn heartbeat() {
    sync_index::run();
}

mod sync_index {
    use super::*;

    pub fn run() {
        if let Some((index_canister_id, args)) = RUNTIME_STATE.with(|state| next_batch(state.borrow_mut().as_mut().unwrap())) {
            ic_cdk::block_on(send_to_index(index_canister_id, args));
        }
    }

    fn next_batch(runtime_state: &mut RuntimeState) -> Option<(CanisterId, Args)> {
        let bytes_remaining = runtime_state.data.blobs.bytes_remaining();
        runtime_state
            .data
            .index_sync_state
            .pop_args_for_next_sync(bytes_remaining)
            .map(|args| (runtime_state.data.index_canister_id, args))
    }

    async fn send_to_index(index_canister_id: CanisterId, args: Args) {
        match index_canister_c2c_client::c2c_sync_bucket(index_canister_id, &args).await {
            Ok(Response::Success(result)) => {
                RUNTIME_STATE.with(|state| handle_success(result, state.borrow_mut().as_mut().unwrap()));
            }
            Err(_) => {
                RUNTIME_STATE.with(|state| handle_error(args, state.borrow_mut().as_mut().unwrap()));
            }
        }
    }

    fn handle_success(result: SuccessResult, runtime_state: &mut RuntimeState) {
        // For each blob that is rejected by the index canister we want to do 2 things -
        // 1. Record the reason against the user so that they can determine what happened
        // 2. Delete any additional data we have held for that blob
        for blob_reference_rejected in result.blob_references_rejected {
            let blob_id = blob_reference_rejected.blob_id;
            let reason = blob_reference_rejected.reason.into();

            if let Some(user_id) = runtime_state.data.blobs.uploaded_by(&blob_id) {
                if let Some(user) = runtime_state.data.users.get_mut(&user_id) {
                    let old_status = user.set_blob_status(blob_id, BlobStatusInternal::Rejected(reason));

                    if let Some(BlobStatusInternal::Uploading(_)) = old_status {
                        runtime_state.data.blobs.remove_pending_blob(&blob_id);
                    } else {
                        runtime_state.data.blobs.remove_blob_reference(user_id, blob_id);
                    }
                }
            }
        }

        runtime_state.data.index_sync_state.mark_sync_completed();
    }

    fn handle_error(args: Args, runtime_state: &mut RuntimeState) {
        runtime_state.data.index_sync_state.mark_sync_failed(args);
    }
}
