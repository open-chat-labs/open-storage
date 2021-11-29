use crate::guards::caller_is_service_principal;
use crate::model::bucket_sync_state::EventToSync;
use crate::UserRecord;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::add_or_update_users::*;

#[update(guard = "caller_is_service_principal")]
#[trace]
fn add_or_update_users(args: Args) -> Response {
    RUNTIME_STATE.with(|state| add_or_update_users_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn add_or_update_users_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    for user_config in args.users {
        if let Some(user) = runtime_state.data.users.get_mut(&user_config.user_id) {
            user.byte_limit = user_config.byte_limit;
        } else {
            runtime_state.data.users.insert(
                user_config.user_id,
                UserRecord {
                    byte_limit: user_config.byte_limit,
                    bytes_used: 0,
                },
            );

            runtime_state
                .data
                .buckets
                .sync_event(EventToSync::UserAdded(user_config.user_id));
        }
    }

    Response::Success
}
