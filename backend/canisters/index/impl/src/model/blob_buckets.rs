use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use types::{CanisterId, Hash};

#[derive(Serialize, Deserialize, Default)]
pub struct BlobBuckets {
    blobs: HashMap<Hash, BlobRecord>,
}

impl BlobBuckets {
    pub fn add(&mut self, hash: Hash, size: u64, bucket: CanisterId) {
        self.blobs
            .entry(hash)
            .and_modify(|b| {
                if !b.buckets.contains(&bucket) {
                    b.buckets.push(bucket);
                }
            })
            .or_insert_with(|| BlobRecord {
                buckets: vec![bucket],
                size,
            });
    }

    pub fn remove(&mut self, hash: Hash, bucket: CanisterId) -> Option<u64> {
        if let Occupied(mut e) = self.blobs.entry(hash) {
            let entry = e.get_mut();
            if let Some(index) = entry.buckets.iter().position(|b| b == &bucket) {
                entry.buckets.remove(index);
                let size = entry.size;
                if entry.buckets.is_empty() {
                    e.remove();
                }
                return Some(size);
            }
        }

        None
    }

    pub fn bucket(&self, hash: &Hash) -> Option<CanisterId> {
        self.blobs.get(hash).map(|r| r.buckets.first().copied()).flatten()
    }

    pub fn get(&self, hash: &Hash) -> Option<&BlobRecord> {
        self.blobs.get(hash)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlobRecord {
    pub buckets: Vec<CanisterId>,
    pub size: u64,
}
