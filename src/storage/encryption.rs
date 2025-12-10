//! AES-GCM encryption for credential storage

use super::StorageError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;

// 32-byte key for AES-256
const ENCRYPTION_KEY: &[u8; 32] = b"docker-registry-mgr-key-v1!!!!!!";

/// Encrypt a string using AES-256-GCM and return base64 encoded result
pub fn encrypt_string(data: &str) -> Result<String, StorageError> {
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
    
    // Generate random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
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
    
    let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
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
