use crate::guards::caller_is_bucket;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::c2c_sync_bucket::*;

#[update(guard = "caller_is_bucket")]
#[trace]
fn c2c_sync_bucket(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_sync_bucket_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_sync_bucket_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let bucket = runtime_state.env.caller();

    let blob_references_rejected = args
        .blob_references_added
        .into_iter()
        .filter_map(|br_added| runtime_state.data.add_blob_reference(bucket, br_added).err())
        .collect();

    for br_removed in args.blob_references_removed {
        runtime_state.data.remove_blob_reference(bucket, br_removed);
    }

    if args.bytes_remaining <= 0 {
        runtime_state.data.buckets.archive(bucket);
    }

    Response::Success(SuccessResult {
        blob_references_rejected,
    })
}
