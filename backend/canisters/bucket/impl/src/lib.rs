use crate::model::blobs::Blobs;
use crate::model::index_sync_state::IndexSyncState;
use crate::model::users::Users;
use canister_logger::LogMessagesWrapper;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use types::{CanisterId, TimestampMillis, Timestamped, Version};
use utils::env::Environment;

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

const MAX_EVENTS_TO_SYNC_PER_BATCH: usize = 1000;
const DATA_LIMIT_BYTES: u64 = 1 << 30; // 1Gb
const MAX_BLOB_SIZE_BYTES: u64 = 100 * (1 << 20); // 100Mb

thread_local! {
    static RUNTIME_STATE: RefCell<Option<RuntimeState>> = RefCell::default();
    static LOG_MESSAGES: RefCell<LogMessagesWrapper> = RefCell::default();
    static WASM_VERSION: RefCell<Timestamped<Version>> = RefCell::default();
}

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }

    pub fn is_caller_index_canister(&self) -> bool {
        let caller = self.env.caller();
        caller == self.data.index_canister_id
    }

    pub fn is_caller_known_user(&self) -> bool {
        let caller = self.env.caller();
        self.data.users.exists(&caller)
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    index_canister_id: CanisterId,
    users: Users,
    blobs: Blobs,
    index_sync_state: IndexSyncState,
    created: TimestampMillis,
}

impl Data {
    pub fn new(index_canister_id: CanisterId, now: TimestampMillis) -> Data {
        Data {
            index_canister_id,
            users: Users::default(),
            blobs: Blobs::default(),
            index_sync_state: IndexSyncState::default(),
            created: now,
        }
    }
}
