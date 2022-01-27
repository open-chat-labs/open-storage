use crate::guards::caller_is_service_principal;
use crate::{mutate_state, read_state, RuntimeState};
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::refresh_reference_counts::{Response::*, *};
use std::collections::HashMap;
use types::{CanisterId, Hash, UserId};

#[update(guard = "caller_is_service_principal")]
#[trace]
async fn refresh_reference_counts(_args: Args) -> Response {
    let buckets = read_state(|state| state.data.buckets.iter().map(|b| b.canister_id).collect());

    match get_reference_counts(buckets).await {
        Ok(reference_counts) => {
            mutate_state(|state| set_reference_counts(reference_counts, state));
            Success
        }
        Err(response) => response,
    }
}

async fn get_reference_counts(buckets: Vec<CanisterId>) -> Result<Vec<(CanisterId, HashMap<Hash, Vec<UserId>>)>, Response> {
    let args = bucket_canister::c2c_reference_counts::Args {};
    let futures = buckets
        .iter()
        .map(|b| bucket_canister_c2c_client::c2c_reference_counts(*b, &args));

    let result: Result<Vec<_>, _> = futures::future::join_all(futures).await.into_iter().collect();

    match result {
        Ok(responses) => Ok(buckets
            .into_iter()
            .zip(responses)
            .map(|(b, r)| {
                let bucket_canister::c2c_reference_counts::Response::Success(result) = r;
                (b, result.reference_counts)
            })
            .collect()),
        Err(error) => Err(InternalError(format!("{:?}", error))),
    }
}

fn set_reference_counts(reference_counts: Vec<(CanisterId, HashMap<Hash, Vec<UserId>>)>, runtime_state: &mut RuntimeState) {
    runtime_state.data.blobs.set_reference_counts(reference_counts);
    for user in runtime_state.data.users.values_mut() {
        user.bytes_used = 0;
    }
    for (_, blob_record) in runtime_state.data.blobs.iter() {
        for user_id in blob_record.uploaded_by.keys() {
            if let Some(user) = runtime_state.data.users.get_mut(user_id) {
                user.bytes_used += blob_record.size;
            }
        }
    }
}
