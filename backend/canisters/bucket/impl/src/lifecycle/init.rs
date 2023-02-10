use crate::lifecycle::{init_env, init_state};
use crate::Data;
use bucket_canister::init::Args;
use canister_api_macros::trace;
use ic_cdk_macros::init;
use tracing::info;
use utils::env::Environment;

#[init]
#[trace]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    let env = init_env();

    let index_canister_id = env.caller();

    let data = Data::new(index_canister_id, env.now(), args.test_mode);

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}
