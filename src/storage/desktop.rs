//! Desktop storage adapter using local file system

use super::{StorageAdapter, StorageError};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Application name for storage directory
const APP_NAME: &str = "docker-registry-manager";

/// Get the application data directory path (cached)
fn get_data_dir() -> &'static PathBuf {
    static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();
    DATA_DIR.get_or_init(|| {
        directories::ProjectDirs::from("", "", APP_NAME)
            .map(|dirs| dirs.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from(format!(".{}", APP_NAME)))
    })
}

/// Desktop storage adapter using local files
pub struct DesktopStorage {
    base_path: PathBuf,
}

impl DesktopStorage {
    /// Create a new desktop storage adapter
    pub fn new() -> Result<Self, StorageError> {
        let base_path = get_data_dir().clone();
        
        fs::create_dir_all(&base_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        Ok(Self { base_path })
    }
    
    fn key_to_path(&self, key: &str) -> PathBuf {
        let safe_key = key.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        self.base_path.join(format!("{}.dat", safe_key))
    }
}

impl Default for DesktopStorage {
    fn default() -> Self {
        Self::new().expect("Failed to create desktop storage")
    }
}

impl StorageAdapter for DesktopStorage {
    fn store(&self, key: &str, data: &[u8]) -> Result<(), StorageError> {
        let path = self.key_to_path(key);
        fs::write(&path, data).map_err(|e| StorageError::IoError(e.to_string()))
    }
    
    fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let path = self.key_to_path(key);
        match fs::read(&path) {
            Ok(data) => Ok(Some(data)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(StorageError::IoError(e.to_string())),
        }
    }
    
    fn remove(&self, key: &str) -> Result<(), StorageError> {
        let path = self.key_to_path(key);
        match fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(StorageError::IoError(e.to_string())),
        }
    }
    
    fn clear(&self) -> Result<(), StorageError> {
        let entries = fs::read_dir(&self.base_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        for entry in entries {
            let path = entry.map_err(|e| StorageError::IoError(e.to_string()))?.path();
            if path.extension().map(|e| e == "dat").unwrap_or(false) {
                fs::remove_file(&path).map_err(|e| StorageError::IoError(e.to_string()))?;
            }
        }
        Ok(())
    }
    
    fn keys(&self) -> Result<Vec<String>, StorageError> {
        let entries = fs::read_dir(&self.base_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        
        let mut keys = Vec::new();
        for entry in entries {
            let path = entry.map_err(|e| StorageError::IoError(e.to_string()))?.path();
            if path.extension().map(|e| e == "dat").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    keys.push(stem.to_string_lossy().to_string());
                }
            }
        }
        Ok(keys)
    }
}
