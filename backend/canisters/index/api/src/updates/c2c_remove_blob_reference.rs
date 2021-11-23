use candid::CandidType;
use serde::Deserialize;
use types::{Hash, UserId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub user_id: UserId,
    pub blob_hash: Option<Hash>,
    pub blob_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    UserNotFound,
}
