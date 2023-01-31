use crate::lifecycle::{init_state, BUFFER_SIZE};
use crate::memory::get_upgrades_memory;
use crate::Data;
use bucket_canister::post_upgrade::Args;
use canister_api_macros::trace;
use canister_logger::LogEntry;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};
use serde::Deserialize;
use tracing::info;
use types::TimestampMillis;
use utils::env::canister::CanisterEnv;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    let env = Box::new(CanisterEnv::new());

    let memory = get_upgrades_memory();
    let reader = BufferedReader::new(BUFFER_SIZE, Reader::new(&memory, 0));

    let (data, log_messages, trace_messages): (Data, Vec<LogMessage>, Vec<LogMessage>) =
        serializer::deserialize(reader).unwrap();

    let logs = log_messages.into_iter().map(|m| m.into()).collect();
    let traces = trace_messages.into_iter().map(|m| m.into()).collect();

    canister_logger::init_with_logs(data.test_mode, logs, traces);

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Post-upgrade complete");
}

#[derive(Deserialize)]
pub struct LogMessage {
    pub timestamp: TimestampMillis,
    #[serde(alias = "json")]
    pub message: String,
}

impl From<LogMessage> for LogEntry {
    fn from(value: LogMessage) -> Self {
        LogEntry {
            timestamp: value.timestamp,
            message: value.message,
        }
    }
}
