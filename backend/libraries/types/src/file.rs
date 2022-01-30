use crate::{FileId, Hash, UserId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FileAdded {
    pub file_id: FileId,
    #[serde(rename(deserialize = "uploaded_by"))]
    pub owner: UserId,
    pub hash: Hash,
    pub size: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FileRemoved {
    pub file_id: FileId,
    #[serde(rename(deserialize = "uploaded_by"))]
    pub owner: UserId,
    pub hash: Hash,
    pub blob_deleted: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct FileRejected {
    pub file_id: FileId,
    pub reason: FileRejectedReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum FileRejectedReason {
    AllowanceExceeded,
    UserNotFound,
}
