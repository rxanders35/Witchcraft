use core::fmt;
use std::collections::HashMap;
use std::env::args;

#[derive(Default)]
pub struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore {
    pub fn new(store: HashMap<String, String>) -> Self {
        Self { store }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Result<String, KVError> {
        match self.store.get(key) {
            Some(k) => Ok(k.to_string()),
            None => Err(KVError::KeyDNE),
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<String, KVError> {
        match self.store.remove(key) {
            Some(k) => Ok(k),
            None => Err(KVError::KeyDNE),
        }
    }
}

#[derive(Debug)]
pub enum KVError {
    KeyDNE,
}

impl fmt::Display for KVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KVError::KeyDNE => write!(f, "Key does not exist!"),
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    println!("Hello, world!");
}
