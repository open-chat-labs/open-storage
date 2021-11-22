use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub principal: Principal,
    pub blob_size: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    UserNotFound,
}
