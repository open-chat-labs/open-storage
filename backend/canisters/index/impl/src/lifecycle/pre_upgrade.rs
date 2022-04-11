use crate::{take_state, LOG_MESSAGES};
use canister_api_macros::trace;
use ic_cdk::api::stable::BufferedStableWriter;
use ic_cdk_macros::pre_upgrade;
use tracing::info;

const BUFFER_SIZE: usize = 16 * 1024 * 1024; // 16MB

#[pre_upgrade]
#[trace]
fn pre_upgrade() {
    info!("Pre-upgrade starting");

    let state = take_state();
    let messages_container = LOG_MESSAGES.with(|l| l.take());

    let log_messages = messages_container.logs.drain_messages();
    let trace_messages = messages_container.traces.drain_messages();

    let stable_state = (state.data, log_messages, trace_messages);
    let writer = BufferedStableWriter::new(BUFFER_SIZE);
    serializer::serialize(&stable_state, writer).unwrap();
}
