use candid::CandidType;
use serde::Deserialize;
use types::{CanisterId, Hash};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub file_hash: Hash,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    UserNotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub reference_counts: Vec<ReferenceCount>,
    pub byte_limit: u64,
    pub bytes_used: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ReferenceCount {
    pub bucket: CanisterId,
    pub count: u32,
}
