use crate::guards::caller_is_known_user;
use crate::model::users::{BlobStatusInternal, IndexSyncComplete};
use crate::{read_state, RuntimeState};
use bucket_canister::blob_status::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use types::{BlobStatus, BlobStatusCompleted, BlobStatusRejected, BlobStatusUploading};

#[update(guard = "caller_is_known_user")]
#[trace]
fn blob_status(args: Args) -> Response {
    read_state(|state| blob_status_impl(args, state))
}

fn blob_status_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    let caller = runtime_state.env.caller();
    let user = runtime_state.data.users.get(&caller).unwrap();

    if let Some(status_internal) = user.blob_status(&args.blob_id) {
        let status = match status_internal {
            BlobStatusInternal::Complete(c) => {
                let blob_reference = runtime_state.data.blobs.blob_reference(&args.blob_id).unwrap_or_else(|| {
                    panic!("Data inconsistency. Blob reference not found. BlobId: {}", args.blob_id);
                });

                BlobStatus::Completed(BlobStatusCompleted {
                    created: blob_reference.created,
                    index_sync_complete: matches!(c, IndexSyncComplete::Yes),
                    mime_type: blob_reference.mime_type.clone(),
                    size: runtime_state.data.blobs.data_size(&blob_reference.hash).unwrap_or_default(),
                })
            }
            BlobStatusInternal::Uploading(c) => {
                let pending_blob = runtime_state.data.blobs.pending_blob(&args.blob_id).unwrap_or_else(|| {
                    panic!("Data inconsistency. Pending blob not found. BlobId: {}", args.blob_id);
                });

                BlobStatus::Uploading(BlobStatusUploading {
                    created: pending_blob.created,
                    index_sync_complete: matches!(c, IndexSyncComplete::Yes),
                    mime_type: pending_blob.mime_type.clone(),
                    size: pending_blob.total_size,
                    chunk_size: pending_blob.chunk_size,
                    chunks_remaining: pending_blob.remaining_chunks.iter().copied().collect(),
                })
            }
            BlobStatusInternal::Rejected(r) => BlobStatus::Rejected(BlobStatusRejected { reason: *r }),
        };

        Success(SuccessResult { status })
    } else {
        NotFound
    }
}
