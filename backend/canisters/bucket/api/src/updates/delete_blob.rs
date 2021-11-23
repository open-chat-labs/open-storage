use candid::CandidType;
use serde::Deserialize;
use types::BlobId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub blob_id: BlobId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    NotAuthorized,
    NotFound,
}
