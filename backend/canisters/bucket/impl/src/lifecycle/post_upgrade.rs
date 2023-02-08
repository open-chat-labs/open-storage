use crate::lifecycle::{init_state, reseed_rng, BUFFER_SIZE};
use crate::memory::get_upgrades_memory;
use crate::Data;
use bucket_canister::post_upgrade::Args;
use canister_api_macros::trace;
use canister_logger::LogEntry;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};
use std::time::Duration;
use tracing::info;
use utils::env::canister::CanisterEnv;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    let env = Box::<CanisterEnv>::default();

    let memory = get_upgrades_memory();
    let reader = BufferedReader::new(BUFFER_SIZE, Reader::new(&memory, 0));

    let (data, logs, traces): (Data, Vec<LogEntry>, Vec<LogEntry>) = serializer::deserialize(reader).unwrap();

    canister_logger::init_with_logs(data.test_mode, logs, traces);

    init_state(env, data, args.wasm_version);

    ic_cdk::timer::set_timer(Duration::default(), reseed_rng);

    info!(version = %args.wasm_version, "Post-upgrade complete");
}
