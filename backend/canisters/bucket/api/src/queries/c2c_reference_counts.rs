use candid::CandidType;
use serde::Deserialize;
use std::collections::HashMap;
use types::{Hash, UserId};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub reference_counts: HashMap<Hash, Vec<UserId>>,
}
