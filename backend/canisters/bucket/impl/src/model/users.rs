use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{BlobId, BlobReferenceRejectedReason, UserId};

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

    pub fn get_mut(&mut self, user_id: &UserId) -> Option<&mut UserRecord> {
        self.users.get_mut(user_id)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserRecord {
    blobs_uploaded: HashMap<BlobId, BlobStatus>,
}

impl UserRecord {
    pub fn blobs_uploaded(&self) -> Vec<BlobId> {
        self.blobs_uploaded.keys().copied().collect()
    }

    pub fn blob_status(&self, blob_id: &BlobId) -> Option<&BlobStatus> {
        self.blobs_uploaded.get(blob_id)
    }

    pub fn set_blob_status(&mut self, blob_id: BlobId, status: BlobStatus) -> Option<BlobStatus> {
        self.blobs_uploaded.insert(blob_id, status)
    }
}

#[derive(Serialize, Deserialize)]
pub enum BlobStatus {
    Complete(IndexSyncComplete),
    Uploading(IndexSyncComplete),
    Rejected(RejectedReason),
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum IndexSyncComplete {
    Yes,
    No,
}

#[derive(Serialize, Deserialize)]
pub enum RejectedReason {
    UserNotFound,
    AllowanceReached,
    HashMismatch,
}

impl From<BlobReferenceRejectedReason> for RejectedReason {
    fn from(reason: BlobReferenceRejectedReason) -> Self {
        match reason {
            BlobReferenceRejectedReason::AllowanceReached => RejectedReason::AllowanceReached,
            BlobReferenceRejectedReason::UserNotFound => RejectedReason::UserNotFound,
        }
    }
}
