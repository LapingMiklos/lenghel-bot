use std::{collections::HashSet, sync::Arc};

use serenity::all::UserId;

use super::threadsafe_storage::ThreadSafeStorage;

#[derive(Clone)]
pub struct SubscriberStorage {
    storage: Arc<ThreadSafeStorage>,
}

const USERS: &'static str = "USERS";

impl SubscriberStorage {
    pub fn new(storage: Arc<ThreadSafeStorage>) -> Self {
        if let Err(_) = storage.load::<HashSet<u64>>(USERS) {
            if let Err(err) = storage.save(USERS, HashSet::<u64>::new()) {
                println!("Unable to access storage: {err}")
            };
        }
        SubscriberStorage { storage }
    }

    pub fn all(&self) -> anyhow::Result<HashSet<UserId>> {
        let users: HashSet<u64> = self.storage.load(USERS)?;
        Ok(users.into_iter().map(UserId::new).collect())
    }

    pub fn add(&self, user: &UserId) -> anyhow::Result<()> {
        let mut users: HashSet<u64> = self.storage.load(USERS)?;
        users.insert(user.get());
        self.storage.save(USERS, users)?;
        Ok(())
    }

    pub fn remove(&self, user: &UserId) -> anyhow::Result<()> {
        let users: HashSet<u64> = self
            .storage
            .load::<HashSet<u64>>(USERS)?
            .into_iter()
            .filter(|u| u == &user.get())
            .collect();

        self.storage.save(USERS, users)?;
        Ok(())
    }
}
