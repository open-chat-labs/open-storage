use sha3::{Digest, Sha3_256};
use types::Hash;

pub fn hash_bytes(value: impl AsRef<[u8]>) -> Hash {
    let mut hasher = Sha3_256::new();
    hasher.update(value.as_ref());
    hasher.finalize().into()
}
