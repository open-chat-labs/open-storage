use candid::CandidType;
use serde::Deserialize;
use types::{AccessorId, Hash, UserId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub users_added: Vec<UserId>,
    pub users_removed: Vec<UserId>,
    pub accessors_removed: Vec<AccessorId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    blob_references_removed: Vec<BlobReferenceRemoved>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct BlobReferenceRemoved {
    pub user_id: UserId,
    pub blob_hash: Hash,
    pub blob_deleted: bool,
}
