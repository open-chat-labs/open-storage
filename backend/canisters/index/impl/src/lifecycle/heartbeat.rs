use crate::{BucketRecord, RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::heartbeat;
use tracing::error;
use types::{CanisterWasm, Cycles};
use utils::canister;
use utils::consts::CREATE_CANISTER_CYCLES_FEE;

const MIN_CYCLES_BALANCE: Cycles = 60_000_000_000_000; // 60T
const BUCKET_CANISTER_INITIAL_CYCLES_BALANCE: Cycles = 50_000_000_000_000; // 50T;
const TARGET_ACTIVE_BUCKETS: usize = 4;

#[heartbeat]
fn heartbeat() {
    ensure_sufficient_active_buckets::run();
}

mod ensure_sufficient_active_buckets {
    use super::*;
    use PrepareResponse::*;

    pub fn run() {
        match RUNTIME_STATE.with(|state| prepare(state.borrow().as_ref().unwrap())) {
            AlreadySufficientActiveBuckets => (),
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
        AlreadySufficientActiveBuckets,
        CyclesBalanceTooLow,
        CreateBucket(CreateBucketArgs),
    }

    fn prepare(runtime_state: &RuntimeState) -> PrepareResponse {
        if runtime_state.data.active_buckets.len() >= TARGET_ACTIVE_BUCKETS {
            return AlreadySufficientActiveBuckets;
        }

        let cycles_required = BUCKET_CANISTER_INITIAL_CYCLES_BALANCE + CREATE_CANISTER_CYCLES_FEE;
        if !cycles_utils::can_spend_cycles(cycles_required, MIN_CYCLES_BALANCE) {
            return CyclesBalanceTooLow;
        }

        CreateBucket(CreateBucketArgs {
            canister_wasm: runtime_state.data.bucket_canister_wasm.clone(),
            cycles_to_use: cycles_required,
            init_canister_args: bucket_canister::init::Args {
                index_canister_id: runtime_state.env.canister_id(),
                wasm_version: runtime_state.data.bucket_canister_wasm.version,
                test_mode: runtime_state.data.test_mode,
            },
        })
    }

    async fn create_bucket(args: CreateBucketArgs) {
        let wasm_arg = candid::encode_one(args.init_canister_args).unwrap();

        let result = canister::create_and_install(
            None,
            args.canister_wasm.module,
            wasm_arg,
            args.cycles_to_use,
        )
        .await;

        if let Ok(canister_id) = result {
            let bucket = BucketRecord::new(canister_id, args.canister_wasm.version);
            RUNTIME_STATE.with(|state| commit(bucket, state.borrow_mut().as_mut().unwrap()))
        }
    }

    fn commit(mut bucket: BucketRecord, runtime_state: &mut RuntimeState) {
        bucket.users_to_sync = runtime_state.data.users.keys().copied().collect();
        runtime_state
            .data
            .active_buckets
            .insert(bucket.canister_id, bucket);
    }
}
