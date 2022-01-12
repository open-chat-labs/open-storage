use canister_client_macros::generate_c2c_call;
use std::cell::Cell;
use tracing::error;
use types::{CanisterId, Cycles, Milliseconds, TimestampMillis};
use utils::time::MINUTE_IN_MS;

const CYCLES_CHECK_INTERVAL: Milliseconds = 10 * MINUTE_IN_MS; // 10 minutes

thread_local! {
    static LAST_CHECKED: Cell<TimestampMillis> = Cell::default();
}

pub fn check_cycles_balance(min_cycles_balance: Cycles, top_up_canister_id: CanisterId, now: TimestampMillis) {
    if is_cycles_check_due(now) && should_notify(min_cycles_balance) {
        ic_cdk::block_on(send_notification(top_up_canister_id));
    }
}

fn should_notify(min_cycles_balance: Cycles) -> bool {
    let cycles_balance: Cycles = ic_cdk::api::canister_balance().into();

    cycles_balance < min_cycles_balance
}

fn is_cycles_check_due(now: TimestampMillis) -> bool {
    LAST_CHECKED.with(|t| {
        if now > t.get() + CYCLES_CHECK_INTERVAL {
            t.set(now);
            true
        } else {
            false
        }
    })
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
