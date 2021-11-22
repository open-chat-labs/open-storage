use candid::CandidType;
use serde::Deserialize;
use types::Hash;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub blob_hash: Hash,
    pub blob_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    AllowanceReached,
}
