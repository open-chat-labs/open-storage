use candid::CandidType;
use serde::Deserialize;
use types::{AccessorId, BlobReferenceRemoved, UserId};

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
    pub blob_references_removed: Vec<BlobReferenceRemoved>,
}
