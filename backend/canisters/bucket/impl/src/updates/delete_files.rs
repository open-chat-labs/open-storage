use crate::model::files::RemoveFileResult;
use crate::model::index_sync_state::EventToSync;
use crate::{mutate_state, RuntimeState};
use bucket_canister::delete_files::*;
use canister_api_macros::trace;
use ic_cdk_macros::update;

#[update]
#[trace]
fn delete_files(args: Args) -> Response {
    mutate_state(|state| delete_files_impl(args, state))
}

fn delete_files_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let caller = runtime_state.env.caller();

    let mut success = Vec::new();
    let mut failures = Vec::new();

    for file_id in args.file_ids {
        match runtime_state.data.files.remove(caller, file_id) {
            RemoveFileResult::Success(f) => {
                runtime_state.data.index_sync_state.enqueue(EventToSync::FileRemoved(f));
                success.push(file_id);
            }
            RemoveFileResult::NotAuthorized => {
                failures.push(DeleteFileFailure {
                    file_id,
                    reason: DeleteFileFailureReason::NotAuthorized,
                });
            }
            RemoveFileResult::NotFound => {
                failures.push(DeleteFileFailure {
                    file_id,
                    reason: DeleteFileFailureReason::NotFound,
                });
            }
        }
    }

    Response { success, failures }
}
