use crate::model::files::Files;
use crate::model::index_sync_state::IndexSyncState;
use crate::model::users::Users;
use candid::CandidType;
use canister_logger::LogMessagesWrapper;
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use types::{CanisterId, Cycles, TimestampMillis, Timestamped, Version};
use utils::env::Environment;
use utils::memory;

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

const DATA_LIMIT_BYTES: u64 = 1 << 30; // 1Gb
const MAX_BLOB_SIZE_BYTES: u64 = 100 * (1 << 20); // 100Mb
const MAX_EVENTS_TO_SYNC_PER_BATCH: usize = 1000;
const MIN_CYCLES_BALANCE: Cycles = 2_000_000_000_000; // 2T
const STATE_VERSION: StateVersion = StateVersion::V1;

#[derive(CandidType, Serialize, Deserialize)]
enum StateVersion {
    V1,
}

thread_local! {
    static LOG_MESSAGES: RefCell<LogMessagesWrapper> = RefCell::default();
    static WASM_VERSION: RefCell<Timestamped<Version>> = RefCell::default();
}

canister_state!(RuntimeState);

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

    pub fn metrics(&self) -> Metrics {
        let file_metrics = self.data.files.metrics();

        Metrics {
            memory_used: memory::used(),
            now: self.env.now(),
            cycles_balance: self.env.cycles_balance(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            file_count: file_metrics.file_count,
            blob_count: file_metrics.blob_count,
            index_sync_queue_length: self.data.index_sync_state.queue_len(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    index_canister_id: CanisterId,
    users: Users,
    #[serde(rename(deserialize = "blobs"))]
    files: Files,
    index_sync_state: IndexSyncState,
    created: TimestampMillis,
    test_mode: bool,
}

impl Data {
    pub fn new(index_canister_id: CanisterId, now: TimestampMillis, test_mode: bool) -> Data {
        Data {
            index_canister_id,
            users: Users::default(),
            files: Files::default(),
            index_sync_state: IndexSyncState::default(),
            created: now,
            test_mode,
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub memory_used: u64,
    pub now: TimestampMillis,
    pub cycles_balance: Cycles,
    pub wasm_version: Version,
    pub file_count: u32,
    pub blob_count: u32,
    pub index_sync_queue_length: u32,
}

pub fn calc_chunk_count(chunk_size: u32, total_size: u64) -> u32 {
    (((total_size - 1) / (chunk_size as u64)) + 1) as u32
}
