use bucket_canister::*;
use canister_client_macros::*;

// Queries
generate_c2c_call!(file_status);

// Updates
generate_c2c_call!(c2c_sync_index);
generate_c2c_call!(delete_file);
generate_c2c_call!(delete_files);
generate_c2c_call!(upload_chunk_v2);
