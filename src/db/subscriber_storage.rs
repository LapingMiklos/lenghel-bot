use std::sync::Arc;

use super::threadsafe_storage::ThreadSafeStorage;

pub struct SubscriberStorage {
    storage: Arc<ThreadSafeStorage>,
}

impl SubscriberStorage {
    pub fn new(storage: Arc<ThreadSafeStorage>) -> Self {
        SubscriberStorage { storage }
    }
}
