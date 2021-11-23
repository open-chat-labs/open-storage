use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::query;
use index_canister::user::*;

#[query]
#[trace]
fn user(_args: Args) -> Response {
    RUNTIME_STATE.with(|state| user_impl(state.borrow().as_ref().unwrap()))
}

fn user_impl(runtime_state: &RuntimeState) -> Response {
    let user_id = runtime_state.env.caller();
    if let Some(user) = runtime_state.data.users.get(&user_id) {
        Response::Success(UserRecord {
            bytes_used: user.bytes_used,
            byte_limit: user.byte_limit,
        })
    } else {
        Response::UserNotFound
    }
}
