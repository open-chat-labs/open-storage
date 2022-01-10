use crate::guards::caller_is_service_principal;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::update_bucket_canister_wasm::{Response::*, *};

#[update(guard = "caller_is_service_principal")]
#[trace]
fn update_bucket_canister_wasm(args: Args) -> Response {
    mutate_state(|state| update_bucket_canister_wasm_impl(args, state))
}

fn update_bucket_canister_wasm_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    if args.bucket_canister_wasm.version <= runtime_state.data.bucket_canister_wasm.version {
        VersionNotHigher
    } else {
        runtime_state.data.bucket_canister_wasm = args.bucket_canister_wasm;
        for canister_id in runtime_state.data.buckets.iter().map(|b| b.canister_id) {
            runtime_state.data.canisters_requiring_upgrade.enqueue(canister_id)
        }
        Success
    }
}
