use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{BlobReferenceAdded, BlobReferenceRejected, BlobReferenceRemoved};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub blob_references_added: Vec<BlobReferenceAdded>,
    pub blob_references_removed: Vec<BlobReferenceRemoved>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub blob_references_rejected: Vec<BlobReferenceRejected>,
}
