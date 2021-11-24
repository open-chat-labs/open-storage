use crate::model::index_sync_queue::EventToSync;
use crate::{RuntimeState, MAX_EVENTS_TO_SYNC_PER_BATCH, RUNTIME_STATE};
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
        if runtime_state.data.index_sync_queue.is_empty() {
            None
        } else {
            let mut args = Args {
                blob_references_added: Vec::new(),
                blob_references_removed: Vec::new(),
            };

            for _ in 0..MAX_EVENTS_TO_SYNC_PER_BATCH {
                if let Some(event) = runtime_state.data.index_sync_queue.take() {
                    match event {
                        EventToSync::BlobReferenceAdded(a) => args.blob_references_added.push(a),
                        EventToSync::BlobReferenceRemoved(r) => args.blob_references_removed.push(r),
                    }
                } else {
                    break;
                }
            }
            Some((runtime_state.data.index_canister_id, args))
        }
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

    fn handle_success(_result: SuccessResult, _runtime_state: &mut RuntimeState) {
        // TODO handle rejected blobs
    }

    fn handle_error(args: Args, runtime_state: &mut RuntimeState) {
        // If syncing the events failed, queue them up to try again
        for added in args.blob_references_added {
            runtime_state
                .data
                .index_sync_queue
                .push(EventToSync::BlobReferenceAdded(added));
        }

        for removed in args.blob_references_removed {
            runtime_state
                .data
                .index_sync_queue
                .push(EventToSync::BlobReferenceRemoved(removed));
        }
    }
}
