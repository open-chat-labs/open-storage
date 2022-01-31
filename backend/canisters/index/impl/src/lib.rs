use crate::model::blobs::Blobs;
use crate::model::buckets::Buckets;
use candid::{CandidType, Principal};
use canister_logger::LogMessagesWrapper;
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use types::{
    CanisterId, CanisterWasm, Cycles, FileAdded, FileRejected, FileRejectedReason, FileRemoved, Timestamped, UserId, Version,
};
use utils::canister::CanistersRequiringUpgrade;
use utils::env::Environment;

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

const DEFAULT_CHUNK_SIZE_BYTES: u32 = 1 << 19; // 1/2 Mb
const MAX_EVENTS_TO_SYNC_PER_BATCH: usize = 10000;
const STATE_VERSION: StateVersion = StateVersion::V1;
const MIN_CYCLES_BALANCE: Cycles = 10_000_000_000_000; // 10T
const BUCKET_CANISTER_TOP_UP_AMOUNT: Cycles = 1_000_000_000_000; // 1T

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
    pub blobs: Blobs,
    pub buckets: Buckets,
    pub canisters_requiring_upgrade: CanistersRequiringUpgrade,
    pub total_cycles_spent_on_canisters: Cycles,
    pub test_mode: bool,
}

impl Data {
    fn new(service_principals: Vec<Principal>, bucket_canister_wasm: CanisterWasm, test_mode: bool) -> Data {
        Data {
            service_principals: service_principals.into_iter().collect(),
            bucket_canister_wasm,
            users: HashMap::new(),
            blobs: Blobs::default(),
            buckets: Buckets::default(),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            total_cycles_spent_on_canisters: 0,
            test_mode,
        }
    }

    pub fn add_file_reference(&mut self, bucket: CanisterId, file: FileAdded) -> Result<(), FileRejected> {
        let FileAdded {
            file_id,
            owner,
            hash,
            size,
        } = file;

        if let Some(user) = self.users.get_mut(&owner) {
            if !self.blobs.user_owns_blob(&owner, &hash) {
                let bytes_used_after_upload = user
                    .bytes_used
                    .checked_add(size)
                    .unwrap_or_else(|| panic!("'bytes_used' overflowed for {}", owner));

                if bytes_used_after_upload > user.byte_limit {
                    return Err(FileRejected {
                        file_id,
                        reason: FileRejectedReason::AllowanceExceeded,
                    });
                } else {
                    user.bytes_used = bytes_used_after_upload;
                }
            }
        } else {
            return Err(FileRejected {
                file_id,
                reason: FileRejectedReason::UserNotFound,
            });
        }

        self.blobs.add(hash, size, owner, bucket);

        Ok(())
    }

    pub fn remove_file_reference(&mut self, bucket: CanisterId, file: FileRemoved) {
        let FileRemoved { owner, hash, .. } = file;

        if let Some(bytes_removed) = self.blobs.remove(hash, owner, bucket) {
            if let Some(user) = self.users.get_mut(&owner) {
                user.bytes_used = user.bytes_used.saturating_sub(bytes_removed);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecord {
    pub byte_limit: u64,
    pub bytes_used: u64,
}
