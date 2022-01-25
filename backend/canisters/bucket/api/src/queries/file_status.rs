use candid::CandidType;
use serde::Deserialize;
use types::{FileId, FileStatus};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub file_id: FileId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    NotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub status: FileStatus,
}
