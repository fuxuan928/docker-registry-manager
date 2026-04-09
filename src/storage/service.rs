//! Storage service for persisting application data

use super::{StorageAdapter, StorageError};
use crate::models::{CacheConfig, RegistryConfig, Theme};

#[cfg(not(target_arch = "wasm32"))]
use super::DesktopStorage;

const REGISTRIES_KEY: &str = "registries";
const THEME_KEY: &str = "theme";
const CACHE_CONFIG_KEY: &str = "cache_config";
const ENCRYPTION_VERIFIER_KEY: &str = "encryption_verifier";
const ENCRYPTION_VERIFIER_PAYLOAD: &str = "docker-registry-manager::verifier";

fn has_configuration_data(has_registries: bool, has_encryption_verifier: bool) -> bool {
    has_registries || has_encryption_verifier
}

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
        let encrypted_registries: Result<Vec<RegistryConfig>, String> =
            registries.iter().map(|r| r.encrypt_for_storage()).collect();

        let encrypted_registries =
            encrypted_registries.map_err(|e| StorageError::EncryptionError(e))?;

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

    /// Persist an encrypted verifier so the password can be recognized on future runs.
    pub fn save_encryption_verifier(&self) -> Result<(), StorageError> {
        let encrypted = super::encrypt_string(ENCRYPTION_VERIFIER_PAYLOAD)?;
        self.adapter
            .store(ENCRYPTION_VERIFIER_KEY, encrypted.as_bytes())
    }

    /// Verify the current encryption key against the persisted verifier.
    pub fn verify_encryption_verifier(&self) -> Result<bool, StorageError> {
        match self.adapter.retrieve(ENCRYPTION_VERIFIER_KEY)? {
            Some(data) => {
                let encrypted = String::from_utf8(data)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                let decrypted = super::decrypt_string(&encrypted)?;
                Ok(decrypted == ENCRYPTION_VERIFIER_PAYLOAD)
            }
            None => Ok(false),
        }
    }

    /// Clear all stored data
    pub fn clear_all(&self) -> Result<(), StorageError> {
        self.adapter.clear()
    }

    /// Check if configuration exists
    pub fn has_config(&self) -> bool {
        let has_registries = self
            .adapter
            .retrieve(REGISTRIES_KEY)
            .map(|o| o.is_some())
            .unwrap_or(false);
        let has_encryption_verifier = self
            .adapter
            .retrieve(ENCRYPTION_VERIFIER_KEY)
            .map(|o| o.is_some())
            .unwrap_or(false);

        has_configuration_data(has_registries, has_encryption_verifier)
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

#[cfg(test)]
mod tests {
    use super::has_configuration_data;

    #[test]
    fn configuration_exists_when_verifier_exists_even_without_registries() {
        assert!(has_configuration_data(false, true));
    }
}
