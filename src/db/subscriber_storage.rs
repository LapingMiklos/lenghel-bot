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
        if let Err(_) = storage.load::<HashSet<UserId>>(USERS) {
            if let Err(err) = storage.save(USERS, HashSet::<UserId>::new()) {
                println!("Unable to access storage: {err}")
            };
        }
        SubscriberStorage { storage }
    }

    pub fn all(&self) -> anyhow::Result<HashSet<UserId>> {
        let users: HashSet<UserId> = self.storage.load(USERS)?;
        Ok(users)
    }

    pub fn add(&self, user: &UserId) -> anyhow::Result<()> {
        let mut users: HashSet<UserId> = self.storage.load(USERS)?;
        users.insert(user.clone());
        self.storage.save(USERS, users)?;
        Ok(())
    }

    pub fn remove(&self, user: &UserId) -> anyhow::Result<()> {
        let users: HashSet<UserId> = self
            .storage
            .load::<HashSet<UserId>>(USERS)?
            .into_iter()
            .filter(|u| u == user)
            .collect();

        self.storage.save(USERS, users)?;
        Ok(())
    }
}
