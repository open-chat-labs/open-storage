use bucket_canister::*;
use canister_client_macros::*;

// Queries

// Updates
generate_c2c_call!(c2c_add_users);
generate_c2c_call!(c2c_remove_accessors);
generate_c2c_call!(c2c_remove_users);
