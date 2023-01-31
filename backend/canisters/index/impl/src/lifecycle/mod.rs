use crate::{init_state as set_state, Data, RuntimeState, WASM_VERSION};
use tracing::info;
use types::{CanisterId, Cycles, Timestamped, Version};
use utils::env::Environment;
use utils::time::MINUTE_IN_MS;

mod heartbeat;
mod init;
mod post_upgrade;
mod pre_upgrade;

const BUFFER_SIZE: usize = 4 * 1024 * 1024; // 4MB

fn init_state(env: Box<dyn Environment>, data: Data, wasm_version: Version) {
    let now = env.now();
    let runtime_state = RuntimeState::new(env, data);

    set_state(runtime_state);
    WASM_VERSION.with(|v| *v.borrow_mut() = Timestamped::new(wasm_version, now));
}

pub fn init_cycles_dispenser_client(cycles_dispenser_canister_id: CanisterId, min_cycles_balance: Cycles) {
    let config = cycles_dispenser_client::Config::new(cycles_dispenser_canister_id)
        .with_interval(5 * MINUTE_IN_MS)
        .with_min_cycles_balance(min_cycles_balance);

    cycles_dispenser_client::start(config);

    info!("Initialized cycles dispenser client");
}
