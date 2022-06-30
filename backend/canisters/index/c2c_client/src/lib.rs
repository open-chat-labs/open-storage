use canister_client_macros::*;
use index_canister::*;

// Queries
generate_c2c_call!(allocated_bucket_v2);
generate_c2c_call!(user);

// Updates
generate_c2c_call!(add_or_update_users);
generate_c2c_call!(add_service_principals);
generate_c2c_call!(c2c_notify_low_balance);
generate_c2c_call!(c2c_sync_bucket);
generate_c2c_call!(remove_accessor);
generate_c2c_call!(remove_user);
generate_c2c_call!(update_bucket_canister_wasm);
generate_c2c_call!(update_user_id);
