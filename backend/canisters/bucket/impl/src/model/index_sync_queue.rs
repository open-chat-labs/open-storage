use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use types::{BlobReferenceAdded, BlobReferenceRemoved};

#[derive(Serialize, Deserialize, Default)]
pub struct IndexSyncQueue {
    queue: VecDeque<EventToSync>,
}

impl IndexSyncQueue {
    pub fn push(&mut self, event: EventToSync) {
        self.queue.push_back(event);
    }

    pub fn take(&mut self) -> Option<EventToSync> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Serialize, Deserialize)]
pub enum EventToSync {
    BlobReferenceAdded(BlobReferenceAdded),
    BlobReferenceRemoved(BlobReferenceRemoved),
}
