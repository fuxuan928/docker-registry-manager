//! cURL command generation

use crate::models::{ApiRequestInfo, AuthConfig};

/// Generate a cURL command for fetching a manifest
pub fn generate_curl_command(base_url: &str, repo: &str, tag: &str, auth: &AuthConfig) -> String {
    let url = format!("{}/v2/{}/manifests/{}", base_url, repo, tag);
    let mut parts = vec!["curl".to_string()];
    
    // Add Accept header for manifest
    parts.push("-H".to_string());
    parts.push("'Accept: application/vnd.docker.distribution.manifest.v2+json'".to_string());
    
    // Add auth header
    match auth {
        AuthConfig::BasicAuth { username, .. } => {
            parts.push("-H".to_string());
            parts.push(format!("'Authorization: Basic <{}>***'", username));
        }
        AuthConfig::BearerToken { .. } => {
            parts.push("-H".to_string());
            parts.push("'Authorization: Bearer ***'".to_string());
        }
        AuthConfig::Anonymous | AuthConfig::TlsCert { .. } => {}
    }
    
    parts.push(format!("'{}'", url));
    parts.join(" ")
}

/// Generate a cURL command from API request info
pub fn generate_curl_command_from_info(info: &ApiRequestInfo) -> String {
    let mut parts = vec!["curl".to_string()];
    
    // Add method if not GET
    if info.method != "GET" {
        parts.push("-X".to_string());
        parts.push(info.method.clone());
    }
    
    // Add headers
    for (key, value) in &info.headers {
        // Skip sensitive headers or mask them
        let display_value = if key.to_lowercase() == "authorization" {
            if value.starts_with("Basic ") {
                "Basic ***".to_string()
            } else if value.starts_with("Bearer ") {
                "Bearer ***".to_string()
            } else {
                "***".to_string()
            }
        } else {
            value.clone()
        };
        parts.push("-H".to_string());
        parts.push(format!("'{}: {}'", key, display_value));
    }
    
    // Add URL (quoted for safety)
    parts.push(format!("'{}'", info.url));
    
    parts.join(" ")
}

/// Create ApiRequestInfo from request details
pub fn create_request_info(
    method: &str,
    url: &str,
    headers: Vec<(String, String)>,
) -> ApiRequestInfo {
    let curl_command = generate_curl_command_from_info(&ApiRequestInfo {
        method: method.to_string(),
        url: url.to_string(),
        headers: headers.clone(),
        status_code: None,
        response_headers: None,
        response_body: None,
        curl_command: String::new(),
    });
    
    ApiRequestInfo {
        method: method.to_string(),
        url: url.to_string(),
        headers,
        status_code: None,
        response_headers: None,
        response_body: None,
        curl_command,
    }
}
