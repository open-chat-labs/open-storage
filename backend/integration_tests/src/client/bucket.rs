use crate::{generate_query_call, generate_update_call};
use bucket_canister::*;

// Queries
generate_query_call!(file_info);
generate_query_call!(file_status);

// Updates
generate_update_call!(delete_file);
generate_update_call!(delete_files);
generate_update_call!(forward_file);
generate_update_call!(upload_chunk_v2);

pub mod happy_path {
    use crate::{tick_many, DEFAULT_MIME_TYPE};
    use candid::Principal;
    use ic_state_machine_tests::StateMachine;
    use serde_bytes::ByteBuf;
    use types::{CanisterId, FileId, TimestampMillis};
    use utils::hasher::hash_bytes;

    pub fn upload_file(
        env: &mut StateMachine,
        sender: Principal,
        canister_id: CanisterId,
        file_id: FileId,
        file: Vec<u8>,
        expiry: Option<TimestampMillis>,
    ) {
        let hash = hash_bytes(&file);
        let chunk_size = 1000;
        let total_size = file.len() as u64;

        for (index, chunk) in file.chunks(chunk_size as usize).enumerate() {
            let response = super::upload_chunk_v2(
                env,
                sender,
                canister_id,
                &bucket_canister::upload_chunk_v2::Args {
                    file_id,
                    hash,
                    mime_type: DEFAULT_MIME_TYPE.to_string(),
                    accessors: vec![],
                    chunk_index: index as u32,
                    chunk_size,
                    total_size,
                    bytes: ByteBuf::from(chunk),
                    expiry,
                },
            );

            assert!(matches!(response, bucket_canister::upload_chunk_v2::Response::Success));
        }

        // Tick a few times to propagate the file to the index and finalize state
        tick_many(env, 10);
    }

    pub fn file_info(
        env: &StateMachine,
        sender: Principal,
        canister_id: CanisterId,
        file_id: FileId,
    ) -> bucket_canister::file_info::SuccessResult {
        let response = super::file_info(env, sender, canister_id, &bucket_canister::file_info::Args { file_id });

        if let bucket_canister::file_info::Response::Success(result) = response {
            result
        } else {
            panic!("'file_info' error: {response:?}");
        }
    }

    pub fn file_exists(env: &StateMachine, sender: Principal, canister_id: CanisterId, file_id: FileId) -> bool {
        let response = super::file_info(env, sender, canister_id, &bucket_canister::file_info::Args { file_id });

        matches!(response, bucket_canister::file_info::Response::Success(_))
    }
}
