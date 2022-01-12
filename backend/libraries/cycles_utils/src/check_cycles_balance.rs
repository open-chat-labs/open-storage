use canister_client_macros::generate_c2c_call;
use tracing::error;
use types::{CanisterId, Cycles};

pub fn check_cycles_balance(min_cycles_balance: Cycles, top_up_canister_id: CanisterId) {
    if should_notify(min_cycles_balance) {
        ic_cdk::block_on(send_notification(top_up_canister_id));
    }
}

fn should_notify(min_cycles_balance: Cycles) -> bool {
    let cycles_balance: Cycles = ic_cdk::api::canister_balance().into();

    cycles_balance < min_cycles_balance
}

async fn send_notification(canister_id: CanisterId) {
    let args = c2c_notify_low_balance::Args {};
    if let Ok(response) = c2c_notify_low_balance(canister_id, &args).await {
        if !matches!(response, c2c_notify_low_balance::Response::Success(_)) {
            error!(?response, "Failed to notify low balance");
        }
    }
}

// This is needed because the 'generate_update_call' macro looks for 'c2c_notify_low_balance::Args'
// and 'c2c_notify_low_balance::Response'
mod c2c_notify_low_balance {
    use types::{NotifyLowBalanceArgs, NotifyLowBalanceResponse};

    pub type Args = NotifyLowBalanceArgs;
    pub type Response = NotifyLowBalanceResponse;
}

generate_c2c_call!(c2c_notify_low_balance);
