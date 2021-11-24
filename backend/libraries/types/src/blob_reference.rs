use crate::{BlobId, Hash, UserId};
use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct BlobReferenceAdded {
    pub blob_id: BlobId,
    pub user_id: UserId,
    pub blob_hash: Hash,
    pub blob_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct BlobReferenceRemoved {
    pub user_id: UserId,
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
