//! Registry configuration models

use serde::{Deserialize, Serialize};

/// Registry configuration for connecting to a Docker Registry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Registry URL (e.g., "https://registry.example.com")
    pub url: String,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Connection status (not serialized)
    #[serde(skip)]
    pub status: ConnectionStatus,
}

/// Authentication configuration options
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthConfig {
    /// No authentication
    Anonymous,
    /// HTTP Basic Authentication
    BasicAuth {
        username: String,
        #[serde(default)]
        password: String,
        /// Encrypted password for storage
        #[serde(default, skip_serializing_if = "String::is_empty")]
        encrypted_password: String,
    },
    /// Bearer token authentication
    BearerToken {
        #[serde(default)]
        token: String,
        /// Encrypted token for storage
        #[serde(default, skip_serializing_if = "String::is_empty")]
        encrypted_token: String,
    },
    /// TLS client certificate (Desktop only)
    TlsCert {
        cert_path: String,
        key_path: String,
    },
}

impl AuthConfig {
    /// Encrypt sensitive fields before saving
    pub fn encrypt_for_storage(&self) -> Result<Self, String> {
        use crate::storage::encryption::encrypt_string;
        
        match self {
            AuthConfig::BasicAuth { username, password, .. } => {
                let encrypted = encrypt_string(password).map_err(|e| e.to_string())?;
                Ok(AuthConfig::BasicAuth {
                    username: username.clone(),
                    password: String::new(), // Clear plaintext
                    encrypted_password: encrypted,
                })
            }
            AuthConfig::BearerToken { token, .. } => {
                let encrypted = encrypt_string(token).map_err(|e| e.to_string())?;
                Ok(AuthConfig::BearerToken {
                    token: String::new(), // Clear plaintext
                    encrypted_token: encrypted,
                })
            }
            other => Ok(other.clone()),
        }
    }
    
    /// Decrypt sensitive fields after loading
    pub fn decrypt_from_storage(&self) -> Result<Self, String> {
        use crate::storage::encryption::decrypt_string;
        
        match self {
            AuthConfig::BasicAuth { username, password, encrypted_password } => {
                // If we have encrypted password, decrypt it
                let decrypted = if !encrypted_password.is_empty() {
                    decrypt_string(encrypted_password).map_err(|e| e.to_string())?
                } else {
                    password.clone()
                };
                Ok(AuthConfig::BasicAuth {
                    username: username.clone(),
                    password: decrypted,
                    encrypted_password: String::new(),
                })
            }
            AuthConfig::BearerToken { token, encrypted_token } => {
                let decrypted = if !encrypted_token.is_empty() {
                    decrypt_string(encrypted_token).map_err(|e| e.to_string())?
                } else {
                    token.clone()
                };
                Ok(AuthConfig::BearerToken {
                    token: decrypted,
                    encrypted_token: String::new(),
                })
            }
            other => Ok(other.clone()),
        }
    }
}

/// Connection status for a registry
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ConnectionStatus {
    #[default]
    Unknown,
    Connected,
    Disconnected,
    Error(String),
}

impl RegistryConfig {
    /// Create a new registry configuration with a generated ID
    pub fn new(name: String, url: String, auth: AuthConfig) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            url,
            auth,
            status: ConnectionStatus::Unknown,
        }
    }
    
    /// Prepare for storage by encrypting sensitive data
    pub fn encrypt_for_storage(&self) -> Result<Self, String> {
        Ok(Self {
            id: self.id.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            auth: self.auth.encrypt_for_storage()?,
            status: ConnectionStatus::Unknown,
        })
    }
    
    /// Restore after loading by decrypting sensitive data
    pub fn decrypt_from_storage(&self) -> Result<Self, String> {
        Ok(Self {
            id: self.id.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            auth: self.auth.decrypt_from_storage()?,
            status: ConnectionStatus::Unknown,
        })
    }
}
