use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub principal: Principal,
    pub byte_limit: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    UserAlreadyExists,
}
