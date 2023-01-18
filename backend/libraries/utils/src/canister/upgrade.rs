use crate::canister;
use candid::CandidType;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister;
use ic_cdk::api::management_canister::main::{CanisterInstallMode, InstallCodeArgument};
use tracing::{error, trace};
use types::{CanisterId, CanisterWasm, Version};

pub struct CanisterToUpgrade<A: CandidType> {
    pub canister_id: CanisterId,
    pub current_wasm_version: Version,
    pub new_wasm: CanisterWasm,
    pub args: A,
}

pub async fn upgrade<A: CandidType>(canister_to_upgrade: CanisterToUpgrade<A>) -> CallResult<()> {
    let canister_id = canister_to_upgrade.canister_id;

    trace!(%canister_id, "Canister upgrade starting");

    canister::stop(canister_id).await?;

    let install_code_args = InstallCodeArgument {
        mode: CanisterInstallMode::Upgrade,
        canister_id,
        wasm_module: canister_to_upgrade.new_wasm.module,
        arg: candid::encode_one(canister_to_upgrade.args).unwrap(),
    };
    let install_code_response: CallResult<()> = management_canister::main::install_code(install_code_args.clone()).await;

    let mut error = None;
    if let Err((code, msg)) = install_code_response {
        error!(
            %canister_id,
            from_wasm_version = %canister_to_upgrade.current_wasm_version,
            to_wasm_version = %canister_to_upgrade.new_wasm.version,
            error_code = code as u8,
            error_message = msg.as_str(),
            "Error calling 'install_code'"
        );
        error = Some((code, msg));
    }

    // Call 'start canister' regardless of if 'install_code' succeeded or not.
    if let Err((code, msg)) = canister::start(canister_id).await {
        error = error.or(Some((code, msg)));
    }

    if let Some(error) = error {
        error!(%canister_id, "Canister upgrade failed");
        Err(error)
    } else {
        trace!(%canister_id, "Canister upgrade completed");
        Ok(())
    }
}
