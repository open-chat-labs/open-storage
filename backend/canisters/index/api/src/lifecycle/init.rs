use candid::{CandidType, Principal};
use serde::Deserialize;
use types::{CanisterWasm, Version};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub service_principals: Vec<Principal>,
    pub bucket_canister_wasm: CanisterWasm,
    pub wasm_version: Version,
    pub test_mode: bool,
}
