use crate::lifecycle::{init_state, BUFFER_SIZE};
use crate::memory::get_upgrades_memory;
use crate::model::index_sync_state::EventToSync;
use crate::Data;
use bucket_canister::post_upgrade::Args;
use canister_api_macros::trace;
use canister_logger::LogEntry;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};
use tracing::info;
use utils::env::canister::CanisterEnv;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    let env = Box::new(CanisterEnv::new());

    let memory = get_upgrades_memory();
    let reader = BufferedReader::new(BUFFER_SIZE, Reader::new(&memory, 0));

    let (mut data, logs, traces): (Data, Vec<LogEntry>, Vec<LogEntry>) = serializer::deserialize(reader).unwrap();

    canister_logger::init_with_logs(data.test_mode, logs, traces);

    for file in data.files.iter_files_added() {
        data.index_sync_state.enqueue(EventToSync::FileAdded(file));
    }

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Post-upgrade complete");
}
