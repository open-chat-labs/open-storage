use crate::lifecycle::{init_cycles_dispenser_client, init_state, reseed_rng};
use crate::Data;
use canister_api_macros::trace;
use ic_cdk_macros::init;
use index_canister::init::Args;
use std::time::Duration;
use tracing::info;
use utils::env::canister::CanisterEnv;

#[init]
#[trace]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    if let Some(config) = &args.cycles_dispenser_config {
        init_cycles_dispenser_client(config.canister_id, config.min_cycles_balance);
    }

    let env = Box::new(CanisterEnv::new_insecure());
    let data = Data::new(
        args.service_principals,
        args.bucket_canister_wasm,
        args.cycles_dispenser_config,
        args.test_mode,
    );

    init_state(env, data, args.wasm_version);

    ic_cdk::timer::set_timer(Duration::default(), reseed_rng);

    info!(version = %args.wasm_version, "Initialization complete");
}
