use crate::{BlobId, Hash, UserId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct BlobReferenceAdded {
    pub uploaded_by: UserId,
    pub blob_id: BlobId,
    pub blob_hash: Hash,
    pub blob_size: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct BlobReferenceRemoved {
    pub uploaded_by: UserId,
    pub blob_hash: Hash,
    pub blob_deleted: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct BlobReferenceRejected {
    pub blob_id: BlobId,
    pub reason: BlobReferenceRejectedReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum BlobReferenceRejectedReason {
    AllowanceReached,
    UserNotFound,
}
