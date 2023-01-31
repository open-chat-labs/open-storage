use crate::lifecycle::BUFFER_SIZE;
use crate::take_state;
use canister_api_macros::trace;
use ic_cdk::api::stable::BufferedStableWriter;
use ic_cdk_macros::pre_upgrade;
use tracing::info;

#[pre_upgrade]
#[trace]
fn pre_upgrade() {
    info!("Pre-upgrade starting");

    let state = take_state();
    let logs = canister_logger::export_logs();
    let traces = canister_logger::export_traces();

    let stable_state = (state.data, logs, traces);

    let writer = BufferedStableWriter::new(BUFFER_SIZE);
    serializer::serialize(&stable_state, writer).unwrap();
}
