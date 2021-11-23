use crate::guards::caller_is_service_principal;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::update_user::*;

#[update(guard = "caller_is_service_principal")]
#[trace]
fn update_user(args: Args) -> Response {
    RUNTIME_STATE.with(|state| update_user_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn update_user_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    if let Some(user) = runtime_state.data.users.get_mut(&args.user_id) {
        if let Some(byte_limit) = args.byte_limit {
            user.byte_limit = byte_limit;
        }
        Response::Success
    } else {
        Response::UserNotFound
    }
}
