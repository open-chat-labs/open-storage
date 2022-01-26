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
    Success(Result),
    AllowanceReached,
    UserNotFound,
    BucketUnavailable,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Result {
    pub canister_id: CanisterId,
    pub chunk_size: u32,
}
