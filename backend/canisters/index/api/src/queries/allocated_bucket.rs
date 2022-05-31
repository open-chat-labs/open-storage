use crate::ProjectedAllowance;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{CanisterId, Hash};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub file_hash: Hash,
    pub file_size: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    AllowanceExceeded(ProjectedAllowance),
    UserNotFound,
    BucketUnavailable,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub canister_id: CanisterId,
    pub chunk_size: u32,
    pub projected_allowance: ProjectedAllowance,
}
