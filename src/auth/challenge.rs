//! WWW-Authenticate header parsing (RFC 7235)

use std::collections::HashMap;

/// Parsed authentication challenge from WWW-Authenticate header
#[derive(Clone, Debug, PartialEq)]
pub struct AuthChallenge {
    /// Authentication scheme (e.g., "Bearer", "Basic")
    pub scheme: String,
    /// Challenge parameters
    pub params: HashMap<String, String>,
}

/// Parse WWW-Authenticate header value
/// 
/// Supports formats like:
/// - `Basic realm="Registry"`
/// - `Bearer realm="https://auth.docker.io/token",service="registry.docker.io",scope="repository:library/alpine:pull"`
pub fn parse_www_authenticate(header: &str) -> Option<AuthChallenge> {
    let header = header.trim();
    if header.is_empty() {
        return None;
    }

    // Split scheme from parameters
    let (scheme, rest) = header.split_once(' ').unwrap_or((header, ""));
    let scheme = scheme.to_string();
    
    let mut params = HashMap::new();
    
    if !rest.is_empty() {
        // Parse key="value" pairs
        let mut remaining = rest.trim();
        while !remaining.is_empty() {
            // Find key
            let key_end = remaining.find('=').unwrap_or(remaining.len());
            let key = remaining[..key_end].trim().to_string();
            remaining = &remaining[key_end..];
            
            if remaining.starts_with('=') {
                remaining = &remaining[1..];
                
                // Parse value (may be quoted)
                let value = if remaining.starts_with('"') {
                    remaining = &remaining[1..];
                    let end = remaining.find('"').unwrap_or(remaining.len());
                    let val = remaining[..end].to_string();
                    remaining = &remaining[(end + 1).min(remaining.len())..];
                    val
                } else {
                    let end = remaining.find(',').unwrap_or(remaining.len());
                    let val = remaining[..end].trim().to_string();
                    remaining = &remaining[end..];
                    val
                };
                
                params.insert(key, value);
            }
            
            // Skip comma separator
            remaining = remaining.trim_start_matches(',').trim();
        }
    }
    
    Some(AuthChallenge { scheme, params })
}

impl AuthChallenge {
    /// Get the realm parameter
    pub fn realm(&self) -> Option<&str> {
        self.params.get("realm").map(|s| s.as_str())
    }
    
    /// Get the service parameter (for Bearer auth)
    pub fn service(&self) -> Option<&str> {
        self.params.get("service").map(|s| s.as_str())
    }
    
    /// Get the scope parameter (for Bearer auth)
    pub fn scope(&self) -> Option<&str> {
        self.params.get("scope").map(|s| s.as_str())
    }
}
