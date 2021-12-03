use crate::guards::caller_is_service_principal;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::add_service_principal::{Response::*, *};

#[update(guard = "caller_is_service_principal")]
#[trace]
fn add_service_principal(args: Args) -> Response {
    RUNTIME_STATE.with(|state| add_service_principal_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn add_service_principal_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    runtime_state.data.service_principals.insert(args.principal);
    Success
}
