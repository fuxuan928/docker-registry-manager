//! Web storage adapter using localStorage/IndexedDB

use super::{StorageAdapter, StorageError};
use gloo_storage::{LocalStorage, Storage};

const STORAGE_PREFIX: &str = "drm_";

/// Web storage adapter using localStorage
pub struct WebStorage;

impl WebStorage {
    /// Create a new web storage adapter
    pub fn new() -> Self {
        Self
    }
    
    fn prefixed_key(&self, key: &str) -> String {
        format!("{}{}", STORAGE_PREFIX, key)
    }
}

impl Default for WebStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageAdapter for WebStorage {
    fn store(&self, key: &str, data: &[u8]) -> Result<(), StorageError> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        LocalStorage::set(self.prefixed_key(key), encoded)
            .map_err(|e| StorageError::IoError(e.to_string()))
    }
    
    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        match LocalStorage::get::<String>(self.prefixed_key(key)) {
            Ok(encoded) => {
                let data = base64::engine::general_purpose::STANDARD
                    .decode(&encoded)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                Ok(Some(data))
            }
            Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => Ok(None),
            Err(e) => Err(StorageError::IoError(e.to_string())),
        }
    }
    
    fn remove(&self, key: &str) -> Result<(), StorageError> {
        LocalStorage::delete(self.prefixed_key(key));
        Ok(())
    }
    
    fn clear(&self) -> Result<(), StorageError> {
        // Get all keys with our prefix and remove them
        let keys = self.keys()?;
        for key in keys {
            self.remove(&key)?;
        }
        Ok(())
    }
    
    fn keys(&self) -> Result<Vec<String>, StorageError> {
        let mut keys = Vec::new();
        let storage = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
            .ok_or(StorageError::NotAvailable)?;
        
        let len = storage.length().map_err(|_| StorageError::NotAvailable)?;
        for i in 0..len {
            if let Ok(Some(key)) = storage.key(i) {
                if let Some(stripped) = key.strip_prefix(STORAGE_PREFIX) {
                    keys.push(stripped.to_string());
                }
            }
        }
        Ok(keys)
    }
}
