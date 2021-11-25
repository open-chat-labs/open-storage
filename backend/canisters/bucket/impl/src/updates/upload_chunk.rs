use crate::guards::caller_is_known_user;
use crate::model::blobs::{PutChunkArgs, PutChunkResult};
use crate::model::index_sync_state::EventToSync;
use crate::model::users::{BlobStatus, IndexSyncComplete, RejectedReason};
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::upload_chunk::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use types::{BlobReferenceRemoved, UserId};

#[update(guard = "caller_is_known_user")]
#[trace]
fn upload_chunk(args: Args) -> Response {
    RUNTIME_STATE.with(|state| upload_chunk_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn upload_chunk_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let user_id: UserId = runtime_state.env.caller();
    let now = runtime_state.env.now();
    let user = runtime_state.data.users.get_mut(&user_id).unwrap();
    let blob_id = args.blob_id;

    let mut index_sync_complete = IndexSyncComplete::No;
    if let Some(status) = user.blob_status(&blob_id) {
        match status {
            BlobStatus::Complete(_) | BlobStatus::Rejected(RejectedReason::HashMismatch) => return BlobAlreadyExists,
            BlobStatus::Rejected(RejectedReason::AllowanceReached) => return AllowanceReached,
            BlobStatus::Rejected(RejectedReason::UserNotFound) => return UserNotFound,
            BlobStatus::Uploading(c) => index_sync_complete = *c,
        }
    } else {
        user.set_blob_status(blob_id, BlobStatus::Uploading(IndexSyncComplete::No));
    }

    match runtime_state.data.blobs.put_chunk(PutChunkArgs::new(user_id, args, now)) {
        PutChunkResult::Success(r) => {
            if r.blob_completed {
                user.set_blob_status(blob_id, BlobStatus::Complete(index_sync_complete));
            }
            if let Some(blob_reference_added) = r.blob_reference_added {
                runtime_state
                    .data
                    .index_sync_state
                    .enqueue(EventToSync::BlobReferenceAdded(blob_reference_added));
            }
            Success
        }
        PutChunkResult::BlobAlreadyExists => BlobAlreadyExists,
        PutChunkResult::ChunkAlreadyExists => ChunkAlreadyExists,
        PutChunkResult::HashMismatch(hm) => {
            // When there is a hash mismatch, the blob has already been removed from the list of
            // pending blobs, so we now need to update the status and tell the index canister to
            // remove the blob reference.
            user.set_blob_status(blob_id, BlobStatus::Rejected(RejectedReason::HashMismatch));

            // We only need to remove the blob reference from the index canister if this blob
            // consists of multiple chunks. If the blob is a single chunk then the Success case of
            // this match statement will never have been reached so the blob reference will not have
            // been added to the index canister.
            if hm.chunk_count > 1 {
                runtime_state
                    .data
                    .index_sync_state
                    .enqueue(EventToSync::BlobReferenceRemoved(BlobReferenceRemoved {
                        uploaded_by: user_id,
                        blob_hash: hm.provided_hash,
                        blob_deleted: !runtime_state.data.blobs.contains_hash(&hm.provided_hash),
                    }));
            }

            HashMismatch
        }
    }
}
