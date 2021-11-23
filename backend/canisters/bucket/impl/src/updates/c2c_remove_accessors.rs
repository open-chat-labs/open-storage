use crate::guards::caller_is_index_canister;
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::c2c_remove_accessors::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_index_canister")]
#[trace]
fn c2c_remove_accessors(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_remove_accessors_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_remove_accessors_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    for accessor_id in args.accessor_ids.iter() {
        runtime_state.data.blobs.remove_accessor(accessor_id);
    }
    Success
}
