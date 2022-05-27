use candid::CandidType;
use serde::Deserialize;
use types::{FileId, Hash};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub file_id: FileId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    NotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub is_owner: bool,
    pub file_size: u64,
    pub file_hash: Hash,
}
