use candid::CandidType;
use serde::Deserialize;
use types::{AccessorId, UserId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub users_added: Vec<UserId>,
    pub users_removed: Vec<UserId>,
    pub accessors_removed: Vec<AccessorId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
}
