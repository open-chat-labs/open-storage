use candid::CandidType;
use serde::Deserialize;
use types::AccessorId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub accessor_ids: Vec<AccessorId>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
}
