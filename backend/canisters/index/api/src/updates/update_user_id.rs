use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::UserId;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub old_user_id: UserId,
    pub new_user_id: UserId,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    UserNotFound,
    UserIdAlreadyExists,
}
