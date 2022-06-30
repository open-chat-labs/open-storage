use crate::guards::caller_is_service_principal;
use crate::model::bucket_sync_state::EventToSync;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::update_user_id::{Response::*, *};

#[update(guard = "caller_is_service_principal")]
#[trace]
fn update_user_id(args: Args) -> Response {
    mutate_state(|state| update_user_id_impl(args, state))
}

fn update_user_id_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    if runtime_state.data.users.contains_key(&args.new_user_id) {
        UserIdAlreadyExists
    } else if let Some(user) = runtime_state.data.users.remove(&args.old_user_id) {
        for hash in user.blobs_owned.iter() {
            runtime_state
                .data
                .blobs
                .update_user_id(hash, args.old_user_id, args.new_user_id);
        }

        runtime_state.data.users.insert(args.new_user_id, user);
        runtime_state
            .data
            .buckets
            .sync_event(EventToSync::UserIdUpdated(args.old_user_id, args.new_user_id));

        Success
    } else {
        UserNotFound
    }
}
