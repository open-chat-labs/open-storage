#![cfg(test)]

use ic_state_machine_tests::StateMachine;

mod allocation_exceeded_tests;
mod client;
mod file_expiry_tests;
mod rng;
mod setup;
mod upload_file_tests;
mod wasms;

const DEFAULT_MIME_TYPE: &str = "test_mime_type";

// Because data is synced between the index and the buckets asynchronously via heartbeat, you often
// need to tick multiple times for everything to finish processing
fn tick_many(env: &mut StateMachine, count: usize) {
    for _ in 0..count {
        env.tick();
    }
}
