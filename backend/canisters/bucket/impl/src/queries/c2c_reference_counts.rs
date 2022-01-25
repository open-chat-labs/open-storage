use crate::guards::caller_is_index_canister;
use crate::{read_state, RuntimeState};
use bucket_canister::c2c_reference_counts::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_index_canister")]
#[trace]
fn c2c_reference_counts(_args: Args) -> Response {
    read_state(c2c_reference_counts_impl)
}

fn c2c_reference_counts_impl(runtime_state: &RuntimeState) -> Response {
    Success(SuccessResult {
        reference_counts: runtime_state.data.files.reference_counts(),
    })
}
