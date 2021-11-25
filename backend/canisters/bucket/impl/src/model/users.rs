use candid::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{BlobId, BlobReferenceRejectedReason};

#[derive(Serialize, Deserialize, Default)]
pub struct Users {
    users: HashMap<Principal, UserRecord>,
}

impl Users {
    pub fn add(&mut self, principal: Principal) -> bool {
        self.users.insert(principal, UserRecord::default()).is_none()
    }

    pub fn remove(&mut self, principal: Principal) -> Option<UserRecord> {
        self.users.remove(&principal)
    }

    pub fn exists(&self, principal: &Principal) -> bool {
        self.users.contains_key(principal)
    }

    pub fn get_mut(&mut self, principal: &Principal) -> Option<&mut UserRecord> {
        self.users.get_mut(principal)
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
