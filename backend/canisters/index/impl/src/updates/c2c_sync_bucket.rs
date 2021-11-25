use crate::guards::caller_is_bucket;
use crate::{RuntimeState, RUNTIME_STATE};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::c2c_sync_bucket::*;
use types::{BlobReferenceRejected, BlobReferenceRejectedReason};

#[update(guard = "caller_is_bucket")]
#[trace]
fn c2c_sync_bucket(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_sync_bucket_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_sync_bucket_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let bucket = runtime_state.env.caller();
    let mut blob_references_rejected = Vec::new();

    for br_added in args.blob_references_added {
        if let Some(user) = runtime_state.data.users.get_mut(&br_added.user_id) {
            if user.bytes_used + br_added.blob_size > user.byte_limit {
                blob_references_rejected.push(BlobReferenceRejected {
                    blob_id: br_added.blob_id,
                    reason: BlobReferenceRejectedReason::AllowanceReached,
                });
                continue;
            } else {
                user.bytes_used += br_added.blob_size;
            }
        } else {
            blob_references_rejected.push(BlobReferenceRejected {
                blob_id: br_added.blob_id,
                reason: BlobReferenceRejectedReason::UserNotFound,
            });
            continue;
        }

        runtime_state
            .data
            .blob_buckets
            .add(br_added.blob_hash, br_added.blob_size, bucket);
    }

    for br_removed in args.blob_references_removed {
        if let Some(blob_size) = runtime_state.data.blob_buckets.remove(br_removed.blob_hash, bucket) {
            if let Some(user) = runtime_state.data.users.get_mut(&br_removed.user_id) {
                user.bytes_used -= blob_size;
            }
        }
    }

    Response::Success(SuccessResult {
        blob_references_rejected,
    })
}
