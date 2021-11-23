use generic_array::{sequence::Split, typenum::U16, GenericArray};
use sha2::{Digest, Sha256};
use types::Hash;

pub fn hash_bytes(value: impl AsRef<[u8]>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.as_ref());
    let hash = hasher.finalize();

    // https://stackoverflow.com/questions/62928240/how-to-get-first-128-bits-of-sha256-as-u128-without-a-result
    let (head, _): (GenericArray<_, U16>, _) = Split::split(hash);
    u128::from_le_bytes(*head.as_ref())
}
