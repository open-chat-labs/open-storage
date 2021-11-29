use crate::utils::{build_ic_agent, build_management_canister, delay, get_canister_wasm};
use crate::CanisterName;
use candid::CandidType;
use ic_agent::identity::BasicIdentity;
use ic_utils::call::AsyncCall;
use ic_utils::interfaces::management_canister::builders::InstallMode;
use ic_utils::interfaces::ManagementCanister;
use ic_utils::Canister;
use types::{CanisterId, CanisterWasm, Version};

pub async fn upgrade_index_canister(identity: BasicIdentity, url: String, index_canister_id: CanisterId, version: Version) {
    let agent = build_ic_agent(url, identity).await;
    let management_canister = build_management_canister(&agent);
    let canister_wasm = get_canister_wasm(CanisterName::Index, version, false);
    let args = index_canister::post_upgrade::Args { wasm_version: version };

    upgrade_wasm(&management_canister, &index_canister_id, &canister_wasm.module, args).await;
    println!("Index canister upgraded");
}

pub async fn upgrade_bucket_canister(identity: BasicIdentity, url: String, index_canister_id: CanisterId, version: Version) {
    let agent = build_ic_agent(url, identity).await;
    let canister_wasm = get_canister_wasm(CanisterName::Bucket, version, true);
    let args = index_canister::update_bucket_canister_wasm::Args {
        bucket_canister_wasm: CanisterWasm {
            version,
            compressed: canister_wasm.compressed,
            module: canister_wasm.module,
        },
    };

    let response = index_canister_client::update_bucket_canister_wasm(&agent, &index_canister_id, &args)
        .await
        .unwrap();

    if !matches!(response, index_canister::update_bucket_canister_wasm::Response::Success) {
        panic!("{:?}", response);
    }
    println!("Bucket canister wasm upgraded to version {}", version);
}

async fn upgrade_wasm<A: CandidType + Send + Sync>(
    management_canister: &Canister<'_, ManagementCanister>,
    canister_id: &CanisterId,
    wasm_bytes: &[u8],
    args: A,
) {
    println!("Stopping canister {}", canister_id);
    management_canister
        .stop_canister(canister_id)
        .call_and_wait(delay())
        .await
        .expect("Failed to stop canister");
    println!("Canister stopped");

    println!("Upgrading wasm for canister {}", canister_id);
    management_canister
        .install_code(canister_id, wasm_bytes)
        .with_mode(InstallMode::Upgrade)
        .with_arg(args)
        .call_and_wait(delay())
        .await
        .expect("Failed to upgrade wasm");
    println!("Wasm upgraded");

    println!("Starting canister {}", canister_id);
    management_canister
        .start_canister(canister_id)
        .call_and_wait(delay())
        .await
        .expect("Failed to start canister");
    println!("Canister started");
}
