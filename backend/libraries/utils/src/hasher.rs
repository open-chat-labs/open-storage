use sha2::{Digest, Sha256};
use types::Hash;

pub fn hash_bytes(value: impl AsRef<[u8]>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.as_ref());
    let bytes: [u8; 32] = hasher.finalize().into();
    bytes.into()
}