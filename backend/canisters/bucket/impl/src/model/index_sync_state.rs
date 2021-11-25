use crate::MAX_EVENTS_TO_SYNC_PER_BATCH;
use index_canister::c2c_sync_bucket::Args;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use types::{BlobReferenceAdded, BlobReferenceRemoved};

// We want to send events to the index in order, so while a sync is in progress we avoid sending
// more events in case the first batch fails and the second succeeds. If a sync fails, the args that
// were sent are stored so that they can be retried again.
#[derive(Serialize, Deserialize, Default)]
pub struct IndexSyncState {
    queue: VecDeque<EventToSync>,
    in_progress: bool,
    args_to_retry: Option<Args>,
}

impl IndexSyncState {
    pub fn enqueue(&mut self, event: EventToSync) {
        self.queue.push_back(event);
    }

    pub fn get_args_for_next_sync(&mut self) -> Option<Args> {
        if self.in_progress {
            None
        } else if let Some(args) = self.args_to_retry.take() {
            self.in_progress = true;
            Some(args)
        } else if self.queue.is_empty() {
            None
        } else {
            let mut args = Args {
                blob_references_added: Vec::new(),
                blob_references_removed: Vec::new(),
            };

            for _ in 0..MAX_EVENTS_TO_SYNC_PER_BATCH {
                if let Some(event) = self.queue.pop_front() {
                    match event {
                        EventToSync::BlobReferenceAdded(a) => args.blob_references_added.push(a),
                        EventToSync::BlobReferenceRemoved(r) => args.blob_references_removed.push(r),
                    }
                } else {
                    break;
                }
            }
            self.in_progress = true;
            Some(args)
        }
    }

    pub fn mark_sync_completed(&mut self) {
        self.in_progress = false;
    }

    pub fn mark_sync_failed(&mut self, args: Args) {
        self.in_progress = false;
        self.args_to_retry = Some(args);
    }
}

#[derive(Serialize, Deserialize)]
pub enum EventToSync {
    BlobReferenceAdded(BlobReferenceAdded),
    BlobReferenceRemoved(BlobReferenceRemoved),
}
