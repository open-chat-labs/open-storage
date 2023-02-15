use crate::client::{create_canister, install_canister};
use crate::rng::random_principal;
use crate::{tick_many, wasms};
use candid::Principal;
use ic_state_machine_tests::StateMachine;
use lazy_static::lazy_static;
use std::sync::Mutex;
use types::{CanisterId, Version};

lazy_static! {
    static ref ENV: Mutex<Vec<TestEnv>> = Mutex::default();
}

pub struct TestEnv {
    pub env: StateMachine,
    pub index_canister_id: CanisterId,
    pub controller: Principal,
}

pub fn setup_env() -> TestEnv {
    if let Some(env) = try_take_existing_env() {
        return env;
    }
    setup_fresh_env()
}

pub fn setup_fresh_env() -> TestEnv {
    let mut env = StateMachine::new();
    let controller = random_principal();
    let index_canister_id = install_service(&mut env, controller);

    TestEnv {
        env,
        index_canister_id,
        controller,
    }
}

pub fn return_env(env: TestEnv) {
    if let Ok(mut e) = ENV.try_lock() {
        e.push(env);
    }
}

fn try_take_existing_env() -> Option<TestEnv> {
    ENV.try_lock().ok().and_then(|mut e| e.pop())
}

fn install_service(env: &mut StateMachine, controller: Principal) -> CanisterId {
    let index_canister_id = create_canister(env, None);
    let user_controller = random_principal();

    let index_canister_wasm = wasms::INDEX.clone();
    let bucket_canister_wasm = wasms::BUCKET.clone();

    let index_init_args = index_canister::init::Args {
        governance_principals: vec![controller],
        user_controllers: vec![user_controller],
        bucket_canister_wasm,
        cycles_dispenser_config: None,
        wasm_version: Version::min(),
        test_mode: true,
    };
    install_canister(env, index_canister_id, index_canister_wasm, index_init_args);

    // Tick a load of times so that all of the buckets have time to get installed
    tick_many(env, 30);

    index_canister_id
}
