use crate::guards::caller_is_index_canister;
use crate::model::blobs::RemoveBlobReferenceResult;
use crate::{RuntimeState, RUNTIME_STATE};
use bucket_canister::c2c_sync_index::{Response::*, *};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use types::BlobReferenceRemoved;

#[update(guard = "caller_is_index_canister")]
#[trace]
fn c2c_sync_index(args: Args) -> Response {
    RUNTIME_STATE.with(|state| c2c_sync_index_impl(args, state.borrow_mut().as_mut().unwrap()))
}

fn c2c_sync_index_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    let mut blob_references_removed: Vec<BlobReferenceRemoved> = Vec::new();

    for user_id in args.users_added {
        runtime_state.data.users.add(user_id);
    }

    for user_id in args.users_removed {
        if let Some(blob_ids) = runtime_state.data.users.remove(user_id) {
            for blob_id in blob_ids {
                match runtime_state.data.blobs.remove_blob_reference(user_id, blob_id) {
                    RemoveBlobReferenceResult::Success(b) => {
                        blob_references_removed.push(BlobReferenceRemoved {
                            user_id,
                            blob_hash: b.hash,
                            blob_deleted: false,
                        });
                    }
                    RemoveBlobReferenceResult::SuccessBlobDeleted(b) => {
                        blob_references_removed.push(BlobReferenceRemoved {
                            user_id,
                            blob_hash: b.hash,
                            blob_deleted: true,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    for accessor_id in args.accessors_removed {
        blob_references_removed.extend(runtime_state.data.blobs.remove_accessor(&accessor_id));
    }

    Success(SuccessResult { blob_references_removed })
}
