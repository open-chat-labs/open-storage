use candid::CandidType;
use serde::Deserialize;
use types::{BlobId, BlobStatus};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub blob_id: BlobId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    NotFound,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
    pub status: BlobStatus,
}
