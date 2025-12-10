//! Error models for the application

use serde::{Deserialize, Serialize};

/// Application error with details
#[derive(Clone, Debug, PartialEq)]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<ErrorDetails>,
    pub recoverable: bool,
}

/// Error codes for categorization
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    NetworkError,
    AuthenticationError,
    NotFound,
    Forbidden,
    RateLimited,
    ServerError,
    ValidationError,
    StorageError,
    ParseError,
}

/// Detailed error information for debugging
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub status_code: Option<u16>,
    pub response_headers: Option<Vec<(String, String)>>,
    pub response_body: Option<String>,
    pub curl_command: Option<String>,
}

/// API request information for debugging
#[derive(Clone, Debug, PartialEq)]
pub struct ApiRequestInfo {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub status_code: Option<u16>,
    pub response_headers: Option<Vec<(String, String)>>,
    pub response_body: Option<String>,
    pub curl_command: String,
}

/// Theme options
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl AppError {
    /// Create a new application error
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            recoverable: true,
        }
    }

    /// Add error details
    pub fn with_details(mut self, details: ErrorDetails) -> Self {
        self.details = Some(details);
        self
    }

    /// Mark as non-recoverable
    pub fn non_recoverable(mut self) -> Self {
        self.recoverable = false;
        self
    }
}
