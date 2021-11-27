use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use types::{CanisterId, Version};

#[derive(CandidType, Serialize, Deserialize)]
pub struct FailedUpgrade {
    pub canister_id: CanisterId,
    pub from_version: Version,
    pub to_version: Version,
}

#[derive(CandidType, Serialize, Deserialize, Default)]
pub struct CanistersRequiringUpgrade {
    pending: VecDeque<CanisterId>,
    upgrade_in_progress: bool,
    failed: VecDeque<FailedUpgrade>,
}

impl CanistersRequiringUpgrade {
    pub fn enqueue(&mut self, canister_id: CanisterId) {
        self.pending.push_back(canister_id);
    }

    pub fn try_take_next(&mut self) -> Option<CanisterId> {
        if self.upgrade_in_progress {
            None
        } else if let Some(canister_id) = self.pending.pop_front() {
            self.upgrade_in_progress = true;
            Some(canister_id)
        } else {
            None
        }
    }

    pub fn mark_success(&mut self) {
        self.upgrade_in_progress = false;
    }

    pub fn mark_failure(&mut self, failed_upgrade: FailedUpgrade) {
        self.upgrade_in_progress = false;
        self.failed.push_back(failed_upgrade);
    }

    pub fn upgrade_in_progress(&self) -> bool {
        self.upgrade_in_progress
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            pending: self.pending.len(),
            upgrade_in_progress: self.upgrade_in_progress,
            failed: self.failed.len(),
        }
    }

    pub fn remove(&mut self, canister_id: &CanisterId) {
        self.pending.retain(|id| id != canister_id);
        self.failed.retain(|pu| &pu.canister_id != canister_id);
    }
}

pub struct Metrics {
    pub pending: usize,
    pub upgrade_in_progress: bool,
    pub failed: usize,
}
