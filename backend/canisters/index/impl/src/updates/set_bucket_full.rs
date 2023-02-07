use crate::guards::caller_is_service_principal;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::set_bucket_full::{Response::*, *};

#[update(guard = "caller_is_service_principal")]
#[trace]
fn set_bucket_full(args: Args) -> Response {
    mutate_state(|state| set_bucket_full_impl(args, state))
}

fn set_bucket_full_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    runtime_state.data.buckets.set_full(args.bucket, args.full);
    Success
}
