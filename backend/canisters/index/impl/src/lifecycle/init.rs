use crate::lifecycle::{init_cycles_dispenser_client, init_env, init_state};
use crate::Data;
use canister_api_macros::trace;
use ic_cdk_macros::init;
use index_canister::init::Args;
use tracing::info;

#[init]
#[trace]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    if let Some(config) = &args.cycles_dispenser_config {
        init_cycles_dispenser_client(config.canister_id, config.min_cycles_balance);
    }

    let env = init_env();
    let data = Data::new(
        args.service_principals,
        args.bucket_canister_wasm,
        args.cycles_dispenser_config,
        args.test_mode,
    );

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}
