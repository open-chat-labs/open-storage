use crate::ProjectedAllowance;
use candid::CandidType;
use serde::Deserialize;
use types::Hash;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub file_hash: Hash,
    pub file_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(ProjectedAllowance),
    AllowanceExceeded(ProjectedAllowance),
    UserNotFound,
}
