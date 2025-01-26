use std::collections::HashMap;
use thiserror::Error;

#[derive(Default)]
pub struct KVStore {
    store: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum KVError {
    #[error("Key '{0}' not found")]
    KeyNotFound(String),
}

impl KVStore {
    pub fn new() -> KVStore {
        KVStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Result<String, KVError> {
        match self.store.get(key) {
            Some(k) => Ok(k.to_string()),
            None => Err(KVError::KeyNotFound(key.to_string())),
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<String, KVError> {
        match self.store.remove(key) {
            Some(k) => Ok(k),
            None => Err(KVError::KeyNotFound(key.to_string())),
        }
    }
}
