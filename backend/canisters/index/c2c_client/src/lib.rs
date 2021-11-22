use canister_client_macros::*;
use index_canister::*;

// Queries

// Updates
generate_c2c_call!(c2c_add_blob_reference);
generate_c2c_call!(c2c_remove_blob_reference);
