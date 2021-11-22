use candid::{CandidType, Principal};
use serde::Deserialize;
use types::Version;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub service_principals: Vec<Principal>,
    pub wasm_version: Version,
    pub test_mode: bool,
}
