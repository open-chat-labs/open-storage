use crate::model::files::RemoveFileResult;
use crate::model::index_sync_state::EventToSync;
use crate::{mutate_state, RuntimeState};
use bucket_canister::delete_file::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update]
#[trace]
fn delete_file(args: Args) -> Response {
    mutate_state(|state| delete_file_impl(args, state))
}

fn delete_file_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();

    match runtime_state.data.files.remove(caller, args.file_id) {
        RemoveFileResult::Success(f) => {
            runtime_state.data.index_sync_state.enqueue(EventToSync::FileRemoved(f));

            Success
        }
        RemoveFileResult::NotAuthorized => NotAuthorized,
        RemoveFileResult::NotFound => NotFound,
    }
}
