use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use types::{CanisterId, Hash, UserId, Version};

#[derive(Serialize, Deserialize, Default)]
pub struct Buckets {
    active_buckets: Vec<BucketRecord>,
    full_buckets: HashMap<CanisterId, BucketRecord>,
}

impl Buckets {
    pub fn get(&self, canister_id: &CanisterId) -> Option<&BucketRecord> {
        self.active_buckets
            .iter()
            .find(|b| &b.canister_id == canister_id)
            .or_else(|| self.full_buckets.get(canister_id))
    }

    pub fn active_count(&self) -> usize {
        self.active_buckets.len()
    }

    pub fn allocate(&self, blob_hash: Hash) -> Option<CanisterId> {
        let bucket_count = self.active_buckets.len();
        if bucket_count == 0 {
            None
        } else {
            // Use a modified modulo of the hash to slightly favour the first bucket
            // so that they don't all run out of space at the same time
            let index = ((blob_hash as usize) % ((bucket_count * 2) + 1)) % bucket_count;
            Some(self.active_buckets[index].canister_id)
        }
    }

    pub fn sync_user(&mut self, user_id: UserId) {
        for bucket in self.active_buckets.iter_mut() {
            bucket.users_to_sync.push_back(user_id);
        }

        for bucket in self.full_buckets.values_mut() {
            bucket.users_to_sync.push_back(user_id);
        }
    }

    pub fn add(&mut self, bucket: BucketRecord) {
        self.active_buckets.push(bucket);
    }
}

#[derive(Serialize, Deserialize)]
pub struct BucketRecord {
    pub canister_id: CanisterId,
    pub wasm_version: Version,
    pub bytes_used: u64,
    pub users_to_sync: VecDeque<UserId>,
}

impl BucketRecord {
    pub fn new(canister_id: CanisterId, wasm_version: Version) -> BucketRecord {
        BucketRecord {
            canister_id,
            wasm_version,
            bytes_used: 0,
            users_to_sync: VecDeque::new(),
        }
    }
}
