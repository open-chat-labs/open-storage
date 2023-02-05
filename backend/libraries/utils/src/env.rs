use candid::Principal;
use types::{CanisterId, Cycles, TimestampMillis};

pub mod canister;
pub mod test;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> CanisterId;
    fn random_u32(&mut self) -> u32;
    fn cycles_balance(&self) -> Cycles;

    fn random_u64(&mut self) -> u64 {
        let left = self.random_u32() as u64;
        let right = self.random_u32() as u64;

        (left << 32) + right
    }
}
