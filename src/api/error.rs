//! API error types

use thiserror::Error;

/// API error types
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Access forbidden")]
    Forbidden,
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

impl ApiError {
    /// Create an error from HTTP status code
    pub fn from_status(status: u16, message: String) -> Self {
        match status {
            401 => ApiError::Unauthorized,
            403 => ApiError::Forbidden,
            404 => ApiError::NotFound(message),
            429 => ApiError::RateLimited(60), // Default retry after
            500..=599 => ApiError::ServerError(message),
            _ => ApiError::NetworkError(message),
        }
    }
}
