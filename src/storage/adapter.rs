//! Storage adapter trait for platform abstraction

use thiserror::Error;

/// Storage error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum StorageError {
    #[error("Storage not available")]
    NotAvailable,
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("IO error: {0}")]
    IoError(String),
}

/// Storage adapter trait for platform-specific persistence
pub trait StorageAdapter {
    /// Store data with a key
    fn store(&self, key: &str, data: &[u8]) -> Result<(), StorageError>;
    
    /// Retrieve data by key
    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    
    /// Remove data by key
    fn remove(&self, key: &str) -> Result<(), StorageError>;
    
    /// Clear all stored data
    fn clear(&self) -> Result<(), StorageError>;
    
    /// List all keys
    fn keys(&self) -> Result<Vec<String>, StorageError>;
}
