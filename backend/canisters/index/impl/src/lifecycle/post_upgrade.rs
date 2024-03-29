use crate::lifecycle::{init_cycles_dispenser_client, init_env, init_state, BUFFER_SIZE};
use crate::memory::get_upgrades_memory;
use crate::Data;
use canister_api_macros::trace;
use canister_logger::LogEntry;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};
use index_canister::post_upgrade::Args;
use tracing::info;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    let env = init_env();

    let memory = get_upgrades_memory();
    let reader = BufferedReader::new(BUFFER_SIZE, Reader::new(&memory, 0));

    let (data, logs, traces): (Data, Vec<LogEntry>, Vec<LogEntry>) = serializer::deserialize(reader).unwrap();

    canister_logger::init_with_logs(data.test_mode, logs, traces);

    if let Some(config) = &data.cycles_dispenser_config {
        init_cycles_dispenser_client(config.canister_id, config.min_cycles_balance);
    }

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Post-upgrade complete");
}
