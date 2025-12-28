//! AES-GCM encryption for credential storage

use super::StorageError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;
use std::sync::OnceLock;

// Key for AES-256 (initialized at runtime)
static ENCRYPTION_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Initialize the encryption key
pub fn init_key(key: [u8; 32]) -> Result<(), StorageError> {
    ENCRYPTION_KEY.set(key)
        .map_err(|_| StorageError::EncryptionError("Encryption key already initialized".to_string()))
}

fn get_key() -> Result<&'static [u8; 32], StorageError> {
    ENCRYPTION_KEY.get()
        .ok_or_else(|| StorageError::EncryptionError("Encryption key not initialized".to_string()))
}

/// Encrypt a string using AES-256-GCM and return base64 encoded result
pub fn encrypt_string(data: &str) -> Result<String, StorageError> {
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let key = get_key()?;
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
    
    // Generate random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
    
    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);
    
    Ok(STANDARD.encode(&result))
}

/// Decrypt a base64 encoded AES-256-GCM encrypted string
pub fn decrypt_string(data: &str) -> Result<String, StorageError> {
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let decoded = STANDARD
        .decode(data)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
    
    if decoded.len() < 12 {
        return Err(StorageError::EncryptionError("Invalid encrypted data".to_string()));
    }
    
    let key = get_key()?;
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
    
    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&decoded[..12]);
    let ciphertext = &decoded[12..];
    
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))
}
