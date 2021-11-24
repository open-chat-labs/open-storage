use candid::CandidType;
use serde::Deserialize;
use types::{BlobId, Hash, UserId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub add_blob_references: Vec<AddBlobReferenceRequest>,
    pub remove_blob_references: Vec<RemoveBlobReferenceRequest>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AddBlobReferenceRequest {
    pub blob_id: BlobId,
    pub user_id: UserId,
    pub blob_hash: Hash,
    pub blob_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RemoveBlobReferenceRequest {
    pub user_id: UserId,
    pub blob_hash: Hash,
    pub blob_deleted: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub add_blob_reference_failures: Vec<AddBlobReferenceFailure>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AddBlobReferenceFailure {
    pub blob_id: BlobId,
    pub reason: AddBlobReferenceFailureReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum AddBlobReferenceFailureReason {
    AllowanceReached,
    UserNotFound,
}
