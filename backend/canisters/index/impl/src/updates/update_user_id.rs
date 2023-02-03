use crate::guards::caller_is_service_principal;
use canister_api_macros::trace;
use ic_cdk_macros::update;
use index_canister::update_user_id::*;

#[update(guard = "caller_is_service_principal")]
#[trace]
fn update_user_id(_args: Args) -> Response {
    // TODO
    panic!();
}
