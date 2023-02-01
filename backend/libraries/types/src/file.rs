use crate::{FileId, Hash, TimestampMillis, UserId};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FileAdded {
    pub file_id: FileId,
    pub owner: UserId,
    pub hash: Hash,
    pub size: u64,
    pub timestamp: TimestampMillis,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FileRemoved {
    pub file_id: FileId,
    pub owner: UserId,
    pub hash: Hash,
    pub blob_deleted: bool,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct FileRejected {
    pub file_id: FileId,
    pub reason: FileRejectedReason,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum FileRejectedReason {
    AllowanceExceeded,
    UserNotFound,
}
