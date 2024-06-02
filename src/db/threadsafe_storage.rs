use std::sync::RwLock;

use serde::{de::DeserializeOwned, Serialize};
use shuttle_persist::{PersistError, PersistInstance};

pub struct ThreadSafeStorage {
    persist_instance: RwLock<PersistInstance>,
}

impl ThreadSafeStorage {
    pub fn new(persist_instance: PersistInstance) -> Self {
        ThreadSafeStorage {
            persist_instance: RwLock::new(persist_instance),
        }
    }

    pub fn save<T: Serialize>(&self, key: &str, value: T) -> Result<(), PersistError> {
        self.persist_instance.write().unwrap().save(key, value)
    }

    pub fn size(&self) -> Result<usize, PersistError> {
        self.persist_instance.read().unwrap().size()
    }

    pub fn list(&self) -> Result<Vec<String>, PersistError> {
        self.persist_instance.read().unwrap().list()
    }

    pub fn clear(&self) -> Result<(), PersistError> {
        self.persist_instance.write().unwrap().clear()
    }

    pub fn remove(&self, key: &str) -> Result<(), PersistError> {
        self.persist_instance.write().unwrap().remove(key)
    }

    pub fn load<T: DeserializeOwned>(&self, key: &str) -> Result<T, PersistError> {
        self.persist_instance.read().unwrap().load(key)
    }
}
