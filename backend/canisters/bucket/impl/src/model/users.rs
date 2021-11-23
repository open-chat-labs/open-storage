use candid::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::BlobId;

#[derive(Serialize, Deserialize, Default)]
pub struct Users {
    users: HashMap<Principal, Vec<BlobId>>,
}

impl Users {
    pub fn add(&mut self, principal: Principal) -> bool {
        self.users.insert(principal, Vec::new()).is_none()
    }

    pub fn remove(&mut self, principal: Principal) -> Option<Vec<BlobId>> {
        self.users.remove(&principal)
    }

    pub fn exists(&self, principal: &Principal) -> bool {
        self.users.contains_key(principal)
    }
}
