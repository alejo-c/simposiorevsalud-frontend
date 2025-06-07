use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

pub struct StorageService;

impl StorageService {
    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        LocalStorage::get(key).ok()
    }

    pub fn set<T>(&self, key: &str, value: T) -> Result<(), String>
    where
        T: Serialize,
    {
        LocalStorage::set(key, value).map_err(|e| format!("Storage error: {:?}", e))
    }

    pub fn remove(&self, key: &str) {
        LocalStorage::delete(key);
    }
}
