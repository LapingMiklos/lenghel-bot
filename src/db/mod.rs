use std::{collections::HashSet, sync::Arc};

use serenity::all::UserId;
use shuttle_shared_db::SerdeJsonOperator;

#[derive(Clone)]
pub struct SubscriberStorage {
    storage: Arc<shuttle_shared_db::SerdeJsonOperator>,
}

const USERS: &'static str = "USERS";

impl SubscriberStorage {
    pub fn new(storage: Arc<SerdeJsonOperator>) -> Self {
        SubscriberStorage { storage }
    }

    pub async fn all(&self) -> anyhow::Result<HashSet<UserId>> {
        let users: HashSet<u64> = self.storage.read_serialized(USERS).await?;
        Ok(users.into_iter().map(UserId::new).collect())
    }

    pub async fn add(&self, user: &UserId) -> anyhow::Result<()> {
        let mut users: HashSet<u64> = self.storage.read_serialized(USERS).await?;
        users.insert(user.get());
        self.storage.write_serialized(&USERS, &users).await?;
        Ok(())
    }

    pub async fn remove(&self, user: &UserId) -> anyhow::Result<()> {
        let users: HashSet<u64> = self
            .storage
            .read_serialized::<HashSet<u64>>(USERS)
            .await?
            .into_iter()
            .filter(|u| u != &user.get())
            .collect();
        self.storage.write_serialized(USERS, &users).await?;
        Ok(())
    }

    pub async fn init(db: &SerdeJsonOperator) -> anyhow::Result<()> {
        match db.read_serialized::<HashSet<u64>>(USERS).await {
            Ok(users) => println!("Subsicrebed users: {:?}", users),
            Err(_) => {
                let users= HashSet::<u64>::new();
                db.write_serialized(USERS, &users).await?;
                println!("Init db with no subscribers");
            }
        }
        Ok(())
    } 
}