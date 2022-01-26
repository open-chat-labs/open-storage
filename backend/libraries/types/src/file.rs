use crate::{FileId, Hash, UserId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FileAdded {
    #[serde(rename(deserialize = "blob_id"))]
    pub file_id: FileId,
    pub uploaded_by: UserId,
    #[serde(rename(deserialize = "blob_hash"))]
    pub hash: Hash,
    #[serde(rename(deserialize = "blob_size"))]
    pub size: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FileRemoved {
    #[serde(default)]
    pub file_id: FileId,
    pub uploaded_by: UserId,
    #[serde(rename(deserialize = "blob_hash"))]
    pub hash: Hash,
    pub blob_deleted: bool,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct FileRejected {
    #[serde(rename(deserialize = "blob_id"))]
    pub file_id: FileId,
    pub reason: FileRejectedReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum FileRejectedReason {
    AllowanceReached,
    UserNotFound,
}
