use crate::guards::caller_is_known_user;
use crate::model::files::ForwardFileResult;
use crate::model::index_sync_state::EventToSync;
use crate::{mutate_state, RuntimeState};
use bucket_canister::forward_file::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update(guard = "caller_is_known_user")]
#[trace]
fn forward_file(args: Args) -> Response {
    mutate_state(|state| forward_file_impl(args, state))
}

fn forward_file_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    let now = runtime_state.env.now();
    let new_file_id = runtime_state.generate_new_file_id();
    let accessors = args.accessors.into_iter().collect();

    match runtime_state
        .data
        .files
        .forward(caller, args.file_id, new_file_id, accessors, now)
    {
        ForwardFileResult::Success(f) => {
            runtime_state.data.index_sync_state.enqueue(EventToSync::FileAdded(f));
            Success(new_file_id)
        }
        ForwardFileResult::NotAuthorized => NotAuthorized,
        ForwardFileResult::NotFound => NotFound,
    }
}
