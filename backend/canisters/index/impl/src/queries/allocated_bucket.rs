use crate::DEFAULT_CHUNK_SIZE;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::query;
use index_canister::allocated_bucket::*;

#[query]
#[trace]
fn allocated_bucket(args: Args) -> Response {
    RUNTIME_STATE.with(|state| allocated_bucket_impl(args, state.borrow().as_ref().unwrap()))
}

fn allocated_bucket_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let user_id = runtime_state.env.caller();
    if let Some(user) = runtime_state.data.users.get(&user_id) {
        if user.bytes_used + args.blob_size > user.byte_limit {
            return Response::AllowanceReached;
        }
    } else {
        return Response::UserNotFound;
    }

    let bucket = runtime_state
        .data
        .blob_buckets
        .bucket(&args.blob_hash)
        .or_else(|| runtime_state.data.buckets.allocate(args.blob_hash));

    if let Some(canister_id) = bucket {
        Response::Success(Result {
            canister_id,
            chunk_size: DEFAULT_CHUNK_SIZE,
        })
    } else {
        Response::BucketUnavailable
    }
}
