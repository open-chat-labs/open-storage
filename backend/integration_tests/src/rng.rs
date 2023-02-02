use candid::Principal;
use rand::RngCore;
use types::FileId;

pub fn random_file_id() -> FileId {
    rand::thread_rng().next_u32() as u128
}

pub fn random_principal() -> Principal {
    let random_bytes = rand::thread_rng().next_u32().to_ne_bytes();

    Principal::from_slice(&random_bytes)
}
