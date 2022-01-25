use candid::CandidType;
use serde::Deserialize;
use types::FileId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub file_id: FileId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    NotAuthorized,
    NotFound,
}
