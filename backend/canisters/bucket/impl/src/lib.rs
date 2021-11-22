use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use utils::env::Environment;

mod lifecycle;
mod model;
mod queries;
mod updates;

thread_local! {
    static RUNTIME_STATE: RefCell<Option<RuntimeState>> = RefCell::default();
}

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {}
