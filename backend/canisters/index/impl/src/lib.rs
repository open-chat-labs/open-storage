use crate::model::blobs::Blobs;
use crate::model::buckets::Buckets;
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use index_canister::init::CyclesDispenserConfig;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use types::{
    CanisterId, CanisterWasm, Cycles, CyclesTopUp, FileAdded, FileRejected, FileRejectedReason, FileRemoved, Hash,
    TimestampMillis, Timestamped, UserId, Version,
};
use utils::canister::{CanistersRequiringUpgrade, FailedUpgradeCount};
use utils::env::Environment;

mod guards;
mod lifecycle;
mod memory;
mod model;
mod queries;
mod updates;

const DEFAULT_CHUNK_SIZE_BYTES: u32 = 1 << 19; // 1/2 Mb
const MAX_EVENTS_TO_SYNC_PER_BATCH: usize = 1000;
const MIN_CYCLES_BALANCE: Cycles = 20_000_000_000_000; // 20T
const BUCKET_CANISTER_TOP_UP_AMOUNT: Cycles = 5_000_000_000_000; // 5T

thread_local! {
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

    pub fn metrics(&self) -> Metrics {
        let blob_metrics = self.data.blobs.metrics();
        let bucket_upgrade_metrics = self.data.canisters_requiring_upgrade.metrics();

        Metrics {
            memory_used: utils::memory::used(),
            now: self.env.now(),
            cycles_balance: self.env.cycles_balance(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            blob_count: blob_metrics.blob_count,
            total_blob_bytes: blob_metrics.total_blob_bytes,
            file_count: blob_metrics.file_count,
            total_file_bytes: blob_metrics.total_file_bytes,
            active_buckets: self.data.buckets.iter_active_buckets().map(|b| b.into()).collect(),
            full_buckets: self.data.buckets.iter_full_buckets().map(|b| b.into()).collect(),
            bucket_upgrades_pending: bucket_upgrade_metrics.pending as u64,
            bucket_upgrades_in_progress: bucket_upgrade_metrics.in_progress as u64,
            bucket_upgrades_failed: bucket_upgrade_metrics.failed,
            bucket_canister_wasm: self.data.bucket_canister_wasm.version,
            cycles_dispenser_config: self.data.cycles_dispenser_config.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub service_principals: HashSet<Principal>,
    pub bucket_canister_wasm: CanisterWasm,
    pub users: HashMap<UserId, UserRecordInternal>,
    pub blobs: Blobs,
    pub buckets: Buckets,
    pub canisters_requiring_upgrade: CanistersRequiringUpgrade,
    pub total_cycles_spent_on_canisters: Cycles,
    pub cycles_dispenser_config: Option<CyclesDispenserConfig>,
    pub test_mode: bool,
}

impl Data {
    fn new(
        service_principals: Vec<Principal>,
        bucket_canister_wasm: CanisterWasm,
        cycles_dispenser_config: Option<CyclesDispenserConfig>,
        test_mode: bool,
    ) -> Data {
        Data {
            service_principals: service_principals.into_iter().collect(),
            bucket_canister_wasm,
            users: HashMap::new(),
            blobs: Blobs::default(),
            buckets: Buckets::default(),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            total_cycles_spent_on_canisters: 0,
            cycles_dispenser_config,
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
                    user.blobs_owned.insert(hash);
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
                user.blobs_owned.remove(&hash);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UserRecordInternal {
    pub byte_limit: u64,
    pub bytes_used: u64,
    pub blobs_owned: HashSet<Hash>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub memory_used: u64,
    pub now: TimestampMillis,
    pub cycles_balance: Cycles,
    pub wasm_version: Version,
    pub blob_count: u64,
    pub total_blob_bytes: u64,
    pub file_count: u64,
    pub total_file_bytes: u64,
    pub active_buckets: Vec<BucketMetrics>,
    pub full_buckets: Vec<BucketMetrics>,
    pub bucket_upgrades_pending: u64,
    pub bucket_upgrades_in_progress: u64,
    pub bucket_upgrades_failed: Vec<FailedUpgradeCount>,
    pub bucket_canister_wasm: Version,
    pub cycles_dispenser_config: Option<CyclesDispenserConfig>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct BucketMetrics {
    pub canister_id: CanisterId,
    pub wasm_version: Version,
    pub bytes_used: u64,
    pub bytes_remaining: i64,
    pub cycle_top_ups: Vec<CyclesTopUp>,
}
