//! Storage service for persisting application data

use crate::models::{CacheConfig, RegistryConfig, Theme};
use super::{StorageAdapter, StorageError};

#[cfg(not(target_arch = "wasm32"))]
use super::DesktopStorage;

const REGISTRIES_KEY: &str = "registries";
const THEME_KEY: &str = "theme";
const CACHE_CONFIG_KEY: &str = "cache_config";

/// Storage service for application data
pub struct StorageService {
    #[cfg(not(target_arch = "wasm32"))]
    adapter: DesktopStorage,
}

impl StorageService {
    /// Create a new storage service
    pub fn new() -> Result<Self, StorageError> {
        Ok(Self {
            #[cfg(not(target_arch = "wasm32"))]
            adapter: DesktopStorage::new()?,
        })
    }
    
    /// Save registries to storage (with encryption for sensitive data)
    pub fn save_registries(&self, registries: &[RegistryConfig]) -> Result<(), StorageError> {
        // Encrypt sensitive data before saving
        let encrypted_registries: Result<Vec<RegistryConfig>, String> = registries
            .iter()
            .map(|r| r.encrypt_for_storage())
            .collect();
        
        let encrypted_registries = encrypted_registries
            .map_err(|e| StorageError::EncryptionError(e))?;
        
        let json = serde_json::to_string(&encrypted_registries)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        self.adapter.store(REGISTRIES_KEY, json.as_bytes())
    }
    
    /// Load registries from storage (with decryption for sensitive data)
    pub fn load_registries(&self) -> Result<Vec<RegistryConfig>, StorageError> {
        match self.adapter.retrieve(REGISTRIES_KEY)? {
            Some(data) => {
                let json = String::from_utf8(data)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                
                let registries: Vec<RegistryConfig> = serde_json::from_str(&json)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                
                // Decrypt sensitive data after loading
                let decrypted_registries: Result<Vec<RegistryConfig>, String> = registries
                    .iter()
                    .map(|r| r.decrypt_from_storage())
                    .collect();
                
                decrypted_registries.map_err(|e| StorageError::EncryptionError(e))
            }
            None => Ok(Vec::new()),
        }
    }
    
    /// Save theme to storage
    pub fn save_theme(&self, theme: &Theme) -> Result<(), StorageError> {
        let json = serde_json::to_string(theme)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        self.adapter.store(THEME_KEY, json.as_bytes())
    }
    
    /// Load theme from storage
    pub fn load_theme(&self) -> Result<Theme, StorageError> {
        match self.adapter.retrieve(THEME_KEY)? {
            Some(data) => {
                let json = String::from_utf8(data)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                serde_json::from_str(&json)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))
            }
            None => Ok(Theme::default()),
        }
    }
    
    /// Save cache config to storage
    pub fn save_cache_config(&self, config: &CacheConfig) -> Result<(), StorageError> {
        let json = serde_json::to_string(config)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        self.adapter.store(CACHE_CONFIG_KEY, json.as_bytes())
    }
    
    /// Load cache config from storage
    pub fn load_cache_config(&self) -> Result<CacheConfig, StorageError> {
        match self.adapter.retrieve(CACHE_CONFIG_KEY)? {
            Some(data) => {
                let json = String::from_utf8(data)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                serde_json::from_str(&json)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))
            }
            None => Ok(CacheConfig::default()),
        }
    }
    
    /// Clear all stored data
    pub fn clear_all(&self) -> Result<(), StorageError> {
        self.adapter.clear()
    }
    
    /// Check if configuration exists
    pub fn has_config(&self) -> bool {
        self.adapter.retrieve(REGISTRIES_KEY).map(|o| o.is_some()).unwrap_or(false)
    }
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new().expect("Failed to create storage service")
    }
}

/// Global storage service instance
static STORAGE: std::sync::OnceLock<StorageService> = std::sync::OnceLock::new();

/// Get the global storage service
pub fn get_storage() -> &'static StorageService {
    STORAGE.get_or_init(|| StorageService::new().expect("Failed to initialize storage"))
}
