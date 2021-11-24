use candid::CandidType;
use serde::Deserialize;
use types::{BlobId, Hash, UserId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub blob_references_added: Vec<BlobReferenceAdded>,
    pub blob_references_removed: Vec<BlobReferenceRemoved>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

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
pub struct SuccessResult {
    pub blob_references_rejected: Vec<RejectedBlobReference>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RejectedBlobReference {
    pub blob_id: BlobId,
    pub reason: RejectedBlobReferenceReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RejectedBlobReferenceReason {
    AllowanceReached,
    UserNotFound,
}
