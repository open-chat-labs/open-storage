use crate::{read_state, RuntimeState, DEFAULT_CHUNK_SIZE_BYTES};
use canister_api_macros::trace;
use ic_cdk_macros::query;
use index_canister::allocated_bucket::Args as ArgsV1;
use index_canister::allocated_bucket_v2::{Response::*, *};

#[query]
#[trace]
fn allocated_bucket(args: ArgsV1) -> Response {
    read_state(|state| allocated_bucket_impl(args.into(), state))
}

#[query]
#[trace]
fn allocated_bucket_v2(args: Args) -> Response {
    read_state(|state| allocated_bucket_impl(args, state))
}

fn allocated_bucket_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let user_id = runtime_state.env.caller();
    if let Some(user) = runtime_state.data.users.get(&user_id) {
        if user.bytes_used + args.file_size > user.byte_limit {
            return AllowanceReached;
        }
    } else {
        return UserNotFound;
    }

    let bucket = runtime_state
        .data
        .blob_buckets
        .bucket(&args.file_hash)
        .or_else(|| runtime_state.data.buckets.allocate(args.file_hash));

    if let Some(canister_id) = bucket {
        Success(Result {
            canister_id,
            chunk_size: DEFAULT_CHUNK_SIZE_BYTES,
        })
    } else {
        BucketUnavailable
    }
}
