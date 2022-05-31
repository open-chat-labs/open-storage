use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{AccessorId, FileRemoved, UserId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub users_added: Vec<UserId>,
    pub users_removed: Vec<UserId>,
    pub accessors_removed: Vec<AccessorId>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub files_removed: Vec<FileRemoved>,
}
