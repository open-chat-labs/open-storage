use crate::model::bucket_sync_state::EventToSync;
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::c2c_sync_index::{Args, Response, SuccessResult};
use ic_cdk_macros::heartbeat;
use tracing::error;
use types::{CanisterId, CanisterWasm, Cycles};
use utils::canister;
use utils::consts::CREATE_CANISTER_CYCLES_FEE;

const MIN_CYCLES_BALANCE: Cycles = 60_000_000_000_000; // 60T
const BUCKET_CANISTER_INITIAL_CYCLES_BALANCE: Cycles = 50_000_000_000_000; // 50T;

#[heartbeat]
fn heartbeat() {
    ensure_sufficient_active_buckets::run();
    sync_users_with_buckets::run();
}

mod ensure_sufficient_active_buckets {
    use super::*;
    use crate::model::buckets::BucketRecord;
    use PrepareResponse::*;

    pub fn run() {
        match RUNTIME_STATE.with(|state| prepare(state.borrow_mut().as_mut().unwrap())) {
            DoNothing => (),
            CyclesBalanceTooLow => error!("Cycles balance too low to add a new bucket"),
            CreateBucket(args) => {
                ic_cdk::block_on(create_bucket(args));
            }
        }
    }

    struct CreateBucketArgs {
        canister_wasm: CanisterWasm,
        cycles_to_use: Cycles,
        init_canister_args: bucket_canister::init::Args,
    }

    enum PrepareResponse {
        DoNothing,
        CyclesBalanceTooLow,
        CreateBucket(CreateBucketArgs),
    }

    fn prepare(runtime_state: &mut RuntimeState) -> PrepareResponse {
        if !runtime_state.data.buckets.try_to_acquire_creation_lock() {
            return DoNothing;
        }

        let cycles_required = BUCKET_CANISTER_INITIAL_CYCLES_BALANCE + CREATE_CANISTER_CYCLES_FEE;
        if !cycles_utils::can_spend_cycles(cycles_required, MIN_CYCLES_BALANCE) {
            return CyclesBalanceTooLow;
        }

        CreateBucket(CreateBucketArgs {
            canister_wasm: runtime_state.data.bucket_canister_wasm.clone(),
            cycles_to_use: cycles_required,
            init_canister_args: bucket_canister::init::Args {
                wasm_version: runtime_state.data.bucket_canister_wasm.version,
                test_mode: runtime_state.data.test_mode,
            },
        })
    }

    async fn create_bucket(args: CreateBucketArgs) {
        let wasm_arg = candid::encode_one(args.init_canister_args).unwrap();

        let result = canister::create_and_install(None, args.canister_wasm.module, wasm_arg, args.cycles_to_use).await;

        if let Ok(canister_id) = result {
            let bucket = BucketRecord::new(canister_id, args.canister_wasm.version);
            RUNTIME_STATE.with(|state| commit(bucket, state.borrow_mut().as_mut().unwrap()))
        }
    }

    fn commit(mut bucket: BucketRecord, runtime_state: &mut RuntimeState) {
        for user_id in runtime_state.data.users.keys() {
            bucket.sync_state.enqueue(EventToSync::UserAdded(*user_id))
        }
        runtime_state.data.buckets.add_bucket_and_release_creation_lock(bucket);
    }
}

mod sync_users_with_buckets {
    use super::*;

    pub fn run() {
        for (canister_id, args) in RUNTIME_STATE.with(|state| next_batch(state.borrow_mut().as_mut().unwrap())) {
            ic_cdk::block_on(send_to_bucket(canister_id, args));
        }
    }

    fn next_batch(runtime_state: &mut RuntimeState) -> Vec<(CanisterId, Args)> {
        runtime_state.data.buckets.pop_args_for_next_sync()
    }

    async fn send_to_bucket(canister_id: CanisterId, args: Args) {
        match bucket_canister_c2c_client::c2c_sync_index(canister_id, &args).await {
            Ok(Response::Success(result)) => {
                RUNTIME_STATE.with(|state| handle_success(canister_id, result, state.borrow_mut().as_mut().unwrap()));
            }
            Err(_) => {
                RUNTIME_STATE.with(|state| handle_error(canister_id, args, state.borrow_mut().as_mut().unwrap()));
            }
        }
    }

    fn handle_success(canister_id: CanisterId, result: SuccessResult, runtime_state: &mut RuntimeState) {
        for br_removed in result.blob_references_removed {
            runtime_state.data.remove_blob_reference(canister_id, br_removed);
        }

        if let Some(bucket) = runtime_state.data.buckets.get_mut(&canister_id) {
            bucket.sync_state.mark_sync_completed();
        }
    }

    fn handle_error(canister_id: CanisterId, args: Args, runtime_state: &mut RuntimeState) {
        if let Some(bucket) = runtime_state.data.buckets.get_mut(&canister_id) {
            bucket.sync_state.mark_sync_failed(args);
        }
    }
}
