use candid::Principal;
use types::{CanisterId, Cycles, TimestampMillis};

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> CanisterId;
    fn cycles_balance(&self) -> Cycles;
}

#[derive(Default)]
pub struct CanisterEnv {}

impl Environment for CanisterEnv {
    fn now(&self) -> TimestampMillis {
        utils::time::now_millis()
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }

    fn canister_id(&self) -> CanisterId {
        ic_cdk::id()
    }

    fn cycles_balance(&self) -> Cycles {
        ic_cdk::api::canister_balance().into()
    }
}

pub struct TestEnv {
    pub now: u64,
    pub caller: Principal,
    pub canister_id: Principal,
    pub cycles_balance: Cycles,
}

impl Environment for TestEnv {
    fn now(&self) -> u64 {
        self.now
    }

    fn caller(&self) -> Principal {
        self.caller
    }

    fn canister_id(&self) -> CanisterId {
        self.canister_id
    }

    fn cycles_balance(&self) -> Cycles {
        self.cycles_balance
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        TestEnv {
            now: 10000,
            caller: Principal::from_slice(&[1]),
            canister_id: Principal::from_slice(&[1, 2, 3]),
            cycles_balance: 1_000_000_000_000,
        }
    }
}
