use bucket_canister::*;
use canister_client_macros::*;

// Queries
generate_c2c_call!(c2c_reference_counts);

// Updates
generate_c2c_call!(c2c_sync_index);
