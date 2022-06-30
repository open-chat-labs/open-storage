use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{FileId, RejectedReason, UserId};

#[derive(Serialize, Deserialize, Default)]
pub struct Users {
    users: HashMap<UserId, UserRecord>,
}

impl Users {
    pub fn add(&mut self, user_id: UserId) -> bool {
        self.users.insert(user_id, UserRecord::default()).is_none()
    }

    pub fn remove(&mut self, user_id: UserId) -> Option<UserRecord> {
        self.users.remove(&user_id)
    }

    pub fn exists(&self, user_id: &UserId) -> bool {
        self.users.contains_key(user_id)
    }

    pub fn get(&self, user_id: &UserId) -> Option<&UserRecord> {
        self.users.get(user_id)
    }

    pub fn get_mut(&mut self, user_id: &UserId) -> Option<&mut UserRecord> {
        self.users.get_mut(user_id)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserRecord {
    #[serde(alias = "files_uploaded")]
    files_owned: HashMap<FileId, FileStatusInternal>,
}

impl UserRecord {
    pub fn files_owned(&self) -> Vec<FileId> {
        self.files_owned.keys().copied().collect()
    }

    pub fn file_status(&self, file_id: &FileId) -> Option<&FileStatusInternal> {
        self.files_owned.get(file_id)
    }

    pub fn set_file_status(&mut self, file_id: FileId, status: FileStatusInternal) -> Option<FileStatusInternal> {
        self.files_owned.insert(file_id, status)
    }
}

#[derive(Serialize, Deserialize)]
pub enum FileStatusInternal {
    Complete(IndexSyncComplete),
    Uploading(IndexSyncComplete),
    Rejected(RejectedReason),
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum IndexSyncComplete {
    Yes,
    No,
}
