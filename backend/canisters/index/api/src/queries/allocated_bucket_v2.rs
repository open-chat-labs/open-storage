use candid::CandidType;
use serde::Deserialize;
use types::{CanisterId, Hash};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub file_hash: Hash,
    pub file_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    AllowanceExceeded(AllowanceExceededResult),
    UserNotFound,
    BucketUnavailable,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub canister_id: CanisterId,
    pub chunk_size: u32,
    pub byte_limit: u64,
    pub bytes_used: u64,
    pub bytes_used_after_upload: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AllowanceExceededResult {
    pub byte_limit: u64,
    pub bytes_used: u64,
    pub bytes_used_after_upload: u64,
}
