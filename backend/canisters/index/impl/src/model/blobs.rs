use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use types::{CanisterId, Hash, UserId};

#[derive(Serialize, Deserialize, Default)]
pub struct Blobs {
    blobs: HashMap<Hash, BlobRecord>,
}

impl Blobs {
    pub fn add(&mut self, hash: Hash, size: u64, user_id: UserId, bucket: CanisterId) {
        let blob_record = self.blobs.entry(hash).or_insert(BlobRecord {
            uploaded_by: HashMap::new(),
            size,
        });
        blob_record.add_reference(user_id, bucket);
    }

    // Returns the size of the blob if the user no longer owns a copy of it, in which case the
    // user's byte allowance can be increased by the size of the blob, else None.
    pub fn remove(&mut self, hash: Hash, user_id: UserId, bucket: CanisterId) -> Option<u64> {
        if let Occupied(mut e) = self.blobs.entry(hash) {
            let blob_record = e.get_mut();

            if blob_record.remove_reference(user_id, bucket) {
                let size = blob_record.size;
                if blob_record.uploaded_by.is_empty() {
                    e.remove();
                }
                return Some(size);
            }
        };
        None
    }

    pub fn bucket(&self, hash: &Hash) -> Option<CanisterId> {
        self.blobs
            .get(hash)
            .map(|b| b.uploaded_by.values().flatten().map(|rc| rc.bucket).next())
            .flatten()
    }

    pub fn has_user_uploaded_blob(&self, user_id: &UserId, hash: &Hash) -> bool {
        self.blobs.get(hash).map_or(false, |b| b.uploaded_by.contains_key(user_id))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlobRecord {
    pub uploaded_by: HashMap<UserId, Vec<ReferenceCount>>,
    pub size: u64,
}

impl BlobRecord {
    pub fn add_reference(&mut self, user_id: UserId, bucket: CanisterId) {
        let reference_counts = self.uploaded_by.entry(user_id).or_default();
        if let Some(reference_count) = reference_counts.iter_mut().find(|rc| rc.bucket == bucket) {
            reference_count.incr();
        } else {
            reference_counts.push(ReferenceCount::new(bucket, 1));
        }
    }

    // Returns true if the user no longer owns a copy of the object, else false
    pub fn remove_reference(&mut self, user_id: UserId, bucket: CanisterId) -> bool {
        let mut removed_from_user = false;
        if let Occupied(mut e) = self.uploaded_by.entry(user_id) {
            let reference_counts = e.get_mut();
            if let Some((index, reference_count)) = reference_counts.iter_mut().enumerate().find(|(_, rc)| rc.bucket == bucket)
            {
                if reference_count.decr() == 0 {
                    reference_counts.remove(index);
                    if reference_counts.is_empty() {
                        e.remove();
                        removed_from_user = true;
                    }
                }
            }
        }
        removed_from_user
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceCount {
    bucket: CanisterId,
    count: u32,
}

impl ReferenceCount {
    fn new(bucket: CanisterId, count: u32) -> ReferenceCount {
        ReferenceCount { bucket, count }
    }

    fn incr(&mut self) -> u32 {
        self.count += 1;
        self.count
    }

    fn decr(&mut self) -> u32 {
        self.count = self.count.saturating_sub(1);
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn add_many_then_remove_many_is_noop() {
        let mut blobs = Blobs::default();

        let hash = [0; 32];
        let size = 100;

        let bucket1 = Principal::from_slice(&[0, 1]);
        let bucket2 = Principal::from_slice(&[0, 2]);

        for i in 0..10 {
            let user_id = Principal::from_slice(&[i]);

            blobs.add(hash, size, user_id, bucket1);
            blobs.add(hash, size, user_id, bucket2);
        }

        assert_eq!(blobs.blobs.keys().copied().collect::<Vec<_>>(), vec![hash]);
        assert_eq!(blobs.blobs.get(&hash).unwrap().uploaded_by.len(), 10);

        for i in 0..10 {
            let user_id = Principal::from_slice(&[i]);

            assert_eq!(blobs.remove(hash, user_id, bucket1), None);
            assert_eq!(blobs.remove(hash, user_id, bucket2), Some(size));
        }

        assert!(blobs.blobs.is_empty());
    }
}
