//! AES-GCM encryption for credential storage

use super::StorageError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngExt;
use std::sync::{OnceLock, RwLock};

// Key for AES-256 (initialized at runtime)
static ENCRYPTION_KEY: OnceLock<RwLock<Option<[u8; 32]>>> = OnceLock::new();

fn encryption_key_slot() -> &'static RwLock<Option<[u8; 32]>> {
    ENCRYPTION_KEY.get_or_init(|| RwLock::new(None))
}

fn store_key(slot: &RwLock<Option<[u8; 32]>>, key: [u8; 32]) -> Result<(), StorageError> {
    let mut guard = slot
        .write()
        .map_err(|_| StorageError::EncryptionError("Encryption key lock poisoned".to_string()))?;
    *guard = Some(key);
    Ok(())
}

/// Initialize the encryption key
pub fn init_key(key: [u8; 32]) -> Result<(), StorageError> {
    store_key(encryption_key_slot(), key)
}

fn get_key() -> Result<[u8; 32], StorageError> {
    let guard = encryption_key_slot()
        .read()
        .map_err(|_| StorageError::EncryptionError("Encryption key lock poisoned".to_string()))?;
    guard
        .as_ref()
        .copied()
        .ok_or_else(|| StorageError::EncryptionError("Encryption key not initialized".to_string()))
}

/// Encrypt a string using AES-256-GCM and return base64 encoded result
pub fn encrypt_string(data: &str) -> Result<String, StorageError> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let key = get_key()?;
    let cipher = Aes256Gcm::new_from_slice(&key)
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
        return Err(StorageError::EncryptionError(
            "Invalid encrypted data".to_string(),
        ));
    }

    let key = get_key()?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&decoded[..12]);
    let ciphertext = &decoded[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| StorageError::EncryptionError(e.to_string()))?;

    String::from_utf8(plaintext).map_err(|e| StorageError::EncryptionError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{decrypt_string, encrypt_string, init_key};
    use std::sync::RwLock;

    #[test]
    fn store_key_replaces_existing_runtime_key() {
        let slot = RwLock::new(Some([1u8; 32]));

        super::store_key(&slot, [9u8; 32]).expect("replacing runtime key should succeed");

        let stored = *slot.read().expect("read lock should succeed");
        assert_eq!(stored, Some([9u8; 32]));
    }

    #[test]
    fn encrypt_and_decrypt_round_trip() {
        let _ = init_key([7u8; 32]);

        let encrypted = encrypt_string("secret-value").expect("encryption should succeed");
        let decrypted = decrypt_string(&encrypted).expect("decryption should succeed");

        assert_eq!(decrypted, "secret-value");
    }
}
