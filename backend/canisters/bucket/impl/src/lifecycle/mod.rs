use crate::{init_state as set_state, Data, RuntimeState, WASM_VERSION};
use types::{Timestamped, Version};
use utils::env::Environment;

mod heartbeat;
mod init;
mod post_upgrade;
mod pre_upgrade;

const BUFFER_SIZE: usize = 16 * 1024 * 1024; // 16MB

fn init_state(env: Box<dyn Environment>, data: Data, wasm_version: Version) {
    let now = env.now();
    let runtime_state = RuntimeState::new(env, data);

    set_state(runtime_state);
    WASM_VERSION.with(|v| *v.borrow_mut() = Timestamped::new(wasm_version, now));
}
