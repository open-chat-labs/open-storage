use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl From<[u8; 32]> for Hash {
    fn from(bytes: [u8; 32]) -> Hash {
        Hash(bytes)
    }
}