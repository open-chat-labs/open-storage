use crate::env::Environment;
use crate::time;
use candid::Principal;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use types::{CanisterId, Cycles, TimestampMillis};

pub struct CanisterEnv {
    rng: StdRng,
}

impl CanisterEnv {
    pub fn new_with_seed(seed: [u8; 32]) -> Self {
        Self {
            rng: StdRng::from_seed(seed),
        }
    }

    pub fn new_insecure() -> Self {
        let mut seed = [0; 32];
        seed[..8].copy_from_slice(&time::now_nanos().to_ne_bytes());
        Self::new_with_seed(seed)
    }
}

impl Environment for CanisterEnv {
    fn now(&self) -> TimestampMillis {
        time::now_millis()
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }

    fn canister_id(&self) -> CanisterId {
        ic_cdk::id()
    }

    fn random_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn cycles_balance(&self) -> Cycles {
        ic_cdk::api::canister_balance().into()
    }
}
