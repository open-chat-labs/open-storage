use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{BlobId, RejectedReason, UserId};

#[derive(Serialize, Deserialize, Default)]
pub struct Users {
    users: HashMap<UserId, UserRecord>,
}

impl Users {
    pub fn add(&mut self, user_id: UserId) -> bool {
        self.users.insert(user_id, UserRecord::default()).is_none()
    }

    pub fn remove(&mut self, user_id: UserId) -> Option<UserRecord> {
        self.users.remove(&user_id)
    }

    pub fn exists(&self, user_id: &UserId) -> bool {
        self.users.contains_key(user_id)
    }

    pub fn get(&self, user_id: &UserId) -> Option<&UserRecord> {
        self.users.get(user_id)
    }

    pub fn get_mut(&mut self, user_id: &UserId) -> Option<&mut UserRecord> {
        self.users.get_mut(user_id)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserRecord {
    blobs_uploaded: HashMap<BlobId, BlobStatusInternal>,
}

impl UserRecord {
    pub fn blobs_uploaded(&self) -> Vec<BlobId> {
        self.blobs_uploaded.keys().copied().collect()
    }

    pub fn blob_status(&self, blob_id: &BlobId) -> Option<&BlobStatusInternal> {
        self.blobs_uploaded.get(blob_id)
    }

    pub fn set_blob_status(&mut self, blob_id: BlobId, status: BlobStatusInternal) -> Option<BlobStatusInternal> {
        self.blobs_uploaded.insert(blob_id, status)
    }
}

#[derive(Serialize, Deserialize)]
pub enum BlobStatusInternal {
    Complete(IndexSyncComplete),
    Uploading(IndexSyncComplete),
    Rejected(RejectedReason),
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum IndexSyncComplete {
    Yes,
    No,
}
