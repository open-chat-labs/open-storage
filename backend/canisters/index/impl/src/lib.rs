use crate::model::buckets::Buckets;
use candid::Principal;
use canister_logger::LogMessagesWrapper;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use types::{CanisterId, CanisterWasm, Hash, Timestamped, UserId, Version};
use utils::env::Environment;

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

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

    pub fn is_caller_service_principal(&self) -> bool {
        let caller = self.env.caller();
        self.data.service_principals.contains(&caller)
    }

    pub fn is_caller_bucket(&self) -> bool {
        let caller = self.env.caller();
        self.data.buckets.get(&caller).is_some()
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub service_principals: HashSet<Principal>,
    pub bucket_canister_wasm: CanisterWasm,
    pub users: HashMap<UserId, UserRecord>,
    pub blobs: HashMap<Hash, BlobRecord>,
    pub buckets: Buckets,
    pub test_mode: bool,
}

impl Data {
    fn new(service_principals: Vec<Principal>, bucket_canister_wasm: CanisterWasm, test_mode: bool) -> Data {
        Data {
            service_principals: service_principals.into_iter().collect(),
            bucket_canister_wasm,
            users: HashMap::new(),
            blobs: HashMap::new(),
            buckets: Buckets::default(),
            test_mode,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserRecord {
    pub byte_limit: u64,
    pub bytes_used: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BlobRecord {
    pub bucket: CanisterId,
    pub size: u64,
}
