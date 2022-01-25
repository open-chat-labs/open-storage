use crate::guards::caller_is_bucket;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::c2c_sync_bucket::*;

#[update(guard = "caller_is_bucket")]
#[trace]
fn c2c_sync_bucket(args: Args) -> Response {
    mutate_state(|state| c2c_sync_bucket_impl(args, state))
}

fn c2c_sync_bucket_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let bucket = runtime_state.env.caller();

    let files_rejected = args
        .files_added
        .into_iter()
        .filter_map(|file| runtime_state.data.add_file_reference(bucket, file).err())
        .collect();

    for file in args.files_removed {
        runtime_state.data.remove_file_reference(bucket, file);
    }

    if args.bytes_remaining <= 0 {
        runtime_state.data.buckets.archive(bucket);
    }

    Response::Success(SuccessResult { files_rejected })
}
