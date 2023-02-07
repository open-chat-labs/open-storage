use crate::guards::caller_is_service_principal;
use crate::model::buckets::BucketRecord;
use crate::read_state;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::add_bucket_canister::{Response::*, *};
use types::{CanisterId, CanisterWasm};
use utils::canister::create_and_install;

#[update(guard = "caller_is_service_principal")]
#[trace]
async fn add_bucket_canister(args: Args) -> Response {
    let InitBucketArgs { wasm, init_args } = match read_state(|state| prepare(args.canister_id, state)) {
        Ok(ok) => ok,
        Err(response) => return response,
    };

    if let Err(error) = create_and_install(Some(args.canister_id), wasm.module, init_args, 0).await {
        InternalError(format!("{error:?}"))
    } else {
        let bucket = BucketRecord::new(args.canister_id, wasm.version);
        mutate_state(|state| state.data.add_bucket(bucket, false));
        Success
    }
}

struct InitBucketArgs {
    wasm: CanisterWasm,
    init_args: Vec<u8>,
}

fn prepare(canister_id: CanisterId, runtime_state: &RuntimeState) -> Result<InitBucketArgs, Response> {
    if runtime_state.data.buckets.get(&canister_id).is_some() {
        Err(BucketAlreadyAdded)
    } else {
        Ok(InitBucketArgs {
            wasm: runtime_state.data.bucket_canister_wasm.clone(),
            init_args: candid::encode_one(&bucket_canister::init::Args {
                wasm_version: runtime_state.data.bucket_canister_wasm.version,
                test_mode: runtime_state.data.test_mode,
            })
            .unwrap(),
        })
    }
}
