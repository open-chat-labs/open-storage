use crate::{read_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::query;
use index_canister::reference_counts::{Response::*, *};

#[query]
#[trace]
fn reference_counts(args: Args) -> Response {
    read_state(|state| reference_counts_impl(args, state))
}

fn reference_counts_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let user_id = runtime_state.env.caller();
    if let Some(user) = runtime_state.data.users.get(&user_id) {
        let reference_counts = runtime_state
            .data
            .blobs
            .reference_counts(&user_id, &args.file_hash)
            .into_iter()
            .map(|rc| rc.into())
            .collect();

        Success(SuccessResult {
            reference_counts,
            bytes_used: user.bytes_used,
            byte_limit: user.byte_limit,
        })
    } else {
        UserNotFound
    }
}
