use crate::guards::caller_is_index_canister;
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::c2c_add_users::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_index_canister")]
#[trace]
fn c2c_add_users(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_add_users_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_add_users_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    for user_id in args.user_ids.into_iter() {
        runtime_state.data.users.add(user_id);
    }
    Success
}
