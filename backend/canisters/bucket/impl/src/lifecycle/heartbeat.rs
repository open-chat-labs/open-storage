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
        runtime_state
            .data
            .index_sync_state
            .get_args_for_next_sync()
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

    fn handle_success(_result: SuccessResult, runtime_state: &mut RuntimeState) {
        runtime_state.data.index_sync_state.mark_sync_completed();
        // TODO handle rejected blobs
    }

    fn handle_error(args: Args, runtime_state: &mut RuntimeState) {
        runtime_state.data.index_sync_state.mark_sync_failed(args);
    }
}
