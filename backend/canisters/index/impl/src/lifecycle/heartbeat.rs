use crate::model::bucket_sync_state::EventToSync;
use crate::{mutate_state, RuntimeState};
use bucket_canister::c2c_sync_index::{Args, Response, SuccessResult};
use ic_cdk_macros::heartbeat;
use tracing::error;
use types::{CanisterId, CanisterWasm, Cycles, Version};

const MAX_CONCURRENT_CANISTER_UPGRADES: u32 = 1;
const MIN_CYCLES_BALANCE: Cycles = 60_000_000_000_000; // 60T
const BUCKET_CANISTER_INITIAL_CYCLES_BALANCE: Cycles = 10_000_000_000_000; // 10T;

#[heartbeat]
fn heartbeat() {
    ensure_sufficient_active_buckets::run();
    sync_users_with_buckets::run();
    upgrade_canisters::run();
}

mod ensure_sufficient_active_buckets {
    use super::*;
    use crate::model::buckets::BucketRecord;
    use utils::canister::create_and_install;
    use utils::consts::CREATE_CANISTER_CYCLES_FEE;
    use PrepareResponse::*;

    pub fn run() {
        match mutate_state(prepare) {
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

        let (cycles_required, min_cycles_balance) = if runtime_state.data.test_mode {
            (
                (BUCKET_CANISTER_INITIAL_CYCLES_BALANCE + CREATE_CANISTER_CYCLES_FEE) / 4,
                MIN_CYCLES_BALANCE / 10,
            )
        } else {
            (
                BUCKET_CANISTER_INITIAL_CYCLES_BALANCE + CREATE_CANISTER_CYCLES_FEE,
                MIN_CYCLES_BALANCE,
            )
        };
        if !cycles_utils::can_spend_cycles(cycles_required, min_cycles_balance) {
            runtime_state.data.buckets.release_creation_lock();
            return CyclesBalanceTooLow;
        }

        CreateBucket(CreateBucketArgs {
            canister_wasm: runtime_state.data.bucket_canister_wasm.decompress(),
            cycles_to_use: cycles_required,
            init_canister_args: bucket_canister::init::Args {
                wasm_version: runtime_state.data.bucket_canister_wasm.version,
                test_mode: runtime_state.data.test_mode,
            },
        })
    }

    async fn create_bucket(args: CreateBucketArgs) {
        let wasm_arg = candid::encode_one(args.init_canister_args).unwrap();

        let result = create_and_install(None, args.canister_wasm.module, wasm_arg, args.cycles_to_use).await;

        if let Ok(canister_id) = result {
            let bucket = BucketRecord::new(canister_id, args.canister_wasm.version);
            mutate_state(|state| commit(bucket, state))
        } else {
            mutate_state(|state| state.data.buckets.release_creation_lock());
        }
    }

    fn commit(mut bucket: BucketRecord, runtime_state: &mut RuntimeState) {
        for user_id in runtime_state.data.users.keys() {
            bucket.sync_state.enqueue(EventToSync::UserAdded(*user_id))
        }
        runtime_state.data.buckets.add_bucket(bucket, true);
    }
}

mod sync_users_with_buckets {
    use super::*;

    pub fn run() {
        for (canister_id, args) in mutate_state(next_batch) {
            ic_cdk::block_on(send_to_bucket(canister_id, args));
        }
    }

    fn next_batch(runtime_state: &mut RuntimeState) -> Vec<(CanisterId, Args)> {
        runtime_state.data.buckets.pop_args_for_next_sync()
    }

    async fn send_to_bucket(canister_id: CanisterId, args: Args) {
        match bucket_canister_c2c_client::c2c_sync_index(canister_id, &args).await {
            Ok(Response::Success(result)) => {
                mutate_state(|state| handle_success(canister_id, result, state));
            }
            Err(_) => {
                mutate_state(|state| handle_error(canister_id, args, state));
            }
        }
    }

    fn handle_success(canister_id: CanisterId, result: SuccessResult, runtime_state: &mut RuntimeState) {
        for file in result.files_removed {
            runtime_state.data.remove_file_reference(canister_id, file);
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

mod upgrade_canisters {
    use super::*;
    use utils::canister::{upgrade, FailedUpgrade};

    type CanisterToUpgrade = utils::canister::CanisterToUpgrade<bucket_canister::post_upgrade::Args>;

    pub fn run() {
        let canisters_to_upgrade = mutate_state(next_batch);
        if !canisters_to_upgrade.is_empty() {
            ic_cdk::block_on(perform_upgrades(canisters_to_upgrade));
        }
    }

    fn next_batch(runtime_state: &mut RuntimeState) -> Vec<CanisterToUpgrade> {
        let count_in_progress = runtime_state.data.canisters_requiring_upgrade.count_in_progress();
        (0..(MAX_CONCURRENT_CANISTER_UPGRADES - count_in_progress))
            // TODO replace this with 'map_while' once we have upgraded to Rust 1.57
            .map(|_| try_get_next(runtime_state))
            .take_while(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect()
    }

    fn try_get_next(runtime_state: &mut RuntimeState) -> Option<CanisterToUpgrade> {
        let canister_id = runtime_state.data.canisters_requiring_upgrade.try_take_next()?;
        let bucket = runtime_state.data.buckets.get(&canister_id)?;
        let new_wasm = runtime_state.data.bucket_canister_wasm.decompress();

        Some(CanisterToUpgrade {
            canister_id,
            current_wasm_version: bucket.wasm_version,
            new_wasm: runtime_state.data.bucket_canister_wasm.decompress(),
            args: bucket_canister::post_upgrade::Args {
                wasm_version: new_wasm.version,
            },
        })
    }

    async fn perform_upgrades(canisters_to_upgrade: Vec<CanisterToUpgrade>) {
        let futures: Vec<_> = canisters_to_upgrade.into_iter().map(perform_upgrade).collect();

        futures::future::join_all(futures).await;
    }

    async fn perform_upgrade(canister_to_upgrade: CanisterToUpgrade) {
        let canister_id = canister_to_upgrade.canister_id;
        let from_version = canister_to_upgrade.current_wasm_version;
        let to_version = canister_to_upgrade.new_wasm.version;

        match upgrade(canister_to_upgrade).await {
            Ok(_) => {
                mutate_state(|state| on_success(canister_id, to_version, state));
            }
            Err(_) => {
                mutate_state(|state| on_failure(canister_id, from_version, to_version, state));
            }
        }
    }

    fn on_success(canister_id: CanisterId, to_version: Version, runtime_state: &mut RuntimeState) {
        if let Some(bucket) = runtime_state.data.buckets.get_mut(&canister_id) {
            bucket.wasm_version = to_version;
        }
        runtime_state.data.canisters_requiring_upgrade.mark_success(&canister_id);
    }

    fn on_failure(canister_id: CanisterId, from_version: Version, to_version: Version, runtime_state: &mut RuntimeState) {
        runtime_state.data.canisters_requiring_upgrade.mark_failure(FailedUpgrade {
            canister_id,
            from_version,
            to_version,
        });
    }
}
