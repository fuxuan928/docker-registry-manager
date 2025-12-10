//! Authentication handlers for Docker Registry API

use crate::models::AuthConfig;
use base64::{engine::general_purpose::STANDARD, Engine};

/// Authentication type enum
#[derive(Clone, Debug, PartialEq)]
pub enum AuthType {
    Anonymous,
    Basic,
    Bearer,
    TlsCert,
}

/// Get the Authorization header value for the given auth config
pub fn get_auth_header(auth: &AuthConfig) -> Option<String> {
    match auth {
        AuthConfig::Anonymous => None,
        AuthConfig::BasicAuth { username, password, .. } => {
            let credentials = format!("{}:{}", username, password);
            let encoded = STANDARD.encode(credentials.as_bytes());
            Some(format!("Basic {}", encoded))
        }
        AuthConfig::BearerToken { token, .. } => Some(format!("Bearer {}", token)),
        AuthConfig::TlsCert { .. } => None, // TLS cert is handled at connection level
    }
}

/// Decode Basic Auth header back to username and password
pub fn decode_basic_auth(header: &str) -> Option<(String, String)> {
    let header = header.strip_prefix("Basic ")?;
    let decoded = STANDARD.decode(header).ok()?;
    let credentials = String::from_utf8(decoded).ok()?;
    let mut parts = credentials.splitn(2, ':');
    let username = parts.next()?.to_string();
    let password = parts.next()?.to_string();
    Some((username, password))
}

/// Get the authentication type for the given auth config
pub fn get_auth_type(auth: &AuthConfig) -> AuthType {
    match auth {
        AuthConfig::Anonymous => AuthType::Anonymous,
        AuthConfig::BasicAuth { .. } => AuthType::Basic,
        AuthConfig::BearerToken { .. } => AuthType::Bearer,
        AuthConfig::TlsCert { .. } => AuthType::TlsCert,
    }
}
