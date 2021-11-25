use crate::{BlobReferenceRejectedReason, TimestampMillis};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum BlobStatus {
    Completed(BlobStatusCompleted),
    Uploading(BlobStatusUploading),
    Rejected(BlobStatusRejected),
}

#[derive(CandidType, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum RejectedReason {
    UserNotFound,
    AllowanceReached,
    HashMismatch,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BlobStatusCompleted {
    pub created: TimestampMillis,
    pub index_sync_complete: bool,
    pub mime_type: String,
    pub size: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BlobStatusUploading {
    pub created: TimestampMillis,
    pub index_sync_complete: bool,
    pub mime_type: String,
    pub size: u64,
    pub chunk_size: u32,
    pub chunks_remaining: Vec<u32>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BlobStatusRejected {
    pub reason: RejectedReason,
}

impl From<BlobReferenceRejectedReason> for RejectedReason {
    fn from(reason: BlobReferenceRejectedReason) -> Self {
        match reason {
            BlobReferenceRejectedReason::AllowanceReached => RejectedReason::AllowanceReached,
            BlobReferenceRejectedReason::UserNotFound => RejectedReason::UserNotFound,
        }
    }
}
