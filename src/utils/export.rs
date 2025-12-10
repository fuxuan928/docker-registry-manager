//! Export/import utilities

use crate::models::{AuthConfig, RegistryConfig, TagInfo};
use serde::{Deserialize, Serialize};

/// Registry config for export (without sensitive data)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportedRegistryConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub auth_type: String,
}

/// Export registry configurations to JSON (excluding credentials)
pub fn export_registries(registries: &[RegistryConfig]) -> String {
    let exported: Vec<ExportedRegistryConfig> = registries
        .iter()
        .map(|r| ExportedRegistryConfig {
            id: r.id.clone(),
            name: r.name.clone(),
            url: r.url.clone(),
            auth_type: match &r.auth {
                AuthConfig::Anonymous => "anonymous".to_string(),
                AuthConfig::BasicAuth { username, .. } => format!("basic:{}", username),
                AuthConfig::BearerToken { .. } => "bearer".to_string(),
                AuthConfig::TlsCert { .. } => "tls".to_string(),
            },
        })
        .collect();
    
    serde_json::to_string_pretty(&exported).unwrap_or_default()
}

/// Check if exported JSON contains any sensitive credentials
pub fn contains_credentials(json: &str) -> bool {
    let lower = json.to_lowercase();
    lower.contains("\"password\"") || 
    lower.contains("\"token\"") ||
    lower.contains(":\"bearer ") ||
    lower.contains(":\"basic ")
}

/// Export tags to JSON format
pub fn export_tags_json(tags: &[TagInfo]) -> String {
    serde_json::to_string_pretty(tags).unwrap_or_default()
}

/// Export tags to CSV format
pub fn export_tags_csv(tags: &[TagInfo]) -> String {
    let mut csv = String::from("name,digest,size\n");
    for tag in tags {
        csv.push_str(&format!("{},{},{}\n", tag.name, tag.digest, tag.size));
    }
    csv
}

/// Check if exported data contains all tags
pub fn export_contains_all_tags(export: &str, tags: &[TagInfo]) -> bool {
    tags.iter().all(|tag| {
        export.contains(&tag.name) && export.contains(&tag.digest)
    })
}

/// Import registry configurations from JSON
/// Returns configs that need credentials to be filled in
pub fn import_registries(json: &str) -> Result<Vec<RegistryConfig>, String> {
    let exported: Vec<ExportedRegistryConfig> = serde_json::from_str(json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    let configs = exported
        .into_iter()
        .map(|e| {
            let auth = parse_auth_type(&e.auth_type);
            RegistryConfig {
                id: e.id,
                name: e.name,
                url: e.url,
                auth,
                status: crate::models::ConnectionStatus::Unknown,
            }
        })
        .collect();
    
    Ok(configs)
}

/// Parse auth type string back to AuthConfig
fn parse_auth_type(auth_type: &str) -> AuthConfig {
    if auth_type == "anonymous" {
        AuthConfig::Anonymous
    } else if let Some(username) = auth_type.strip_prefix("basic:") {
        AuthConfig::BasicAuth {
            username: username.to_string(),
            password: String::new(), // Needs to be filled in
            encrypted_password: String::new(),
        }
    } else if auth_type == "bearer" {
        AuthConfig::BearerToken {
            token: String::new(), // Needs to be filled in
            encrypted_token: String::new(),
        }
    } else if auth_type == "tls" {
        AuthConfig::TlsCert {
            cert_path: String::new(),
            key_path: String::new(),
        }
    } else {
        AuthConfig::Anonymous
    }
}
