//! Cache configuration and data models

use serde::{Deserialize, Serialize};

/// Cache configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Auto-refresh interval in seconds (0 = disabled)
    pub refresh_interval: u64,
    /// Maximum cache age in seconds
    pub max_age: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            refresh_interval: 0,
            max_age: 3600, // 1 hour
        }
    }
}

/// Cached data wrapper with timestamp
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedData<T> {
    pub data: T,
    pub timestamp: u64,
    pub registry_id: String,
}

impl<T> CachedData<T> {
    /// Create new cached data with current timestamp
    pub fn new(data: T, registry_id: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            data,
            timestamp,
            registry_id,
        }
    }

    /// Check if cache is expired based on max_age
    pub fn is_expired(&self, max_age: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now - self.timestamp > max_age
    }
}
