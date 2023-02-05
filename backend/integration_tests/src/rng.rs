use candid::Principal;
use rand::RngCore;

pub fn random_principal() -> Principal {
    let random_bytes = rand::thread_rng().next_u32().to_ne_bytes();

    Principal::from_slice(&random_bytes)
}
