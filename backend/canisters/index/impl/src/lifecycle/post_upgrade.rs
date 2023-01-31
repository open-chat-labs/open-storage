use crate::lifecycle::{init_cycles_dispenser_client, init_state, BUFFER_SIZE};
use crate::Data;
use canister_api_macros::trace;
use canister_logger::LogEntry;
use ic_cdk::api::stable::BufferedStableReader;
use ic_cdk_macros::post_upgrade;
use index_canister::post_upgrade::Args;
use serde::Deserialize;
use tracing::info;
use types::TimestampMillis;
use utils::env::canister::CanisterEnv;

#[post_upgrade]
#[trace]
fn post_upgrade(args: Args) {
    let env = Box::new(CanisterEnv::new());
    let reader = BufferedStableReader::new(BUFFER_SIZE);

    let (data, log_messages, trace_messages): (Data, Vec<LogMessage>, Vec<LogMessage>) =
        serializer::deserialize(reader).unwrap();

    let logs = log_messages.into_iter().map(|m| m.into()).collect();
    let traces = trace_messages.into_iter().map(|m| m.into()).collect();

    canister_logger::init_with_logs(data.test_mode, logs, traces);

    if let Some(config) = &data.cycles_dispenser_config {
        init_cycles_dispenser_client(config.canister_id, config.min_cycles_balance);
    }

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
