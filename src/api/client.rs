//! Docker Registry API client

use crate::auth::get_auth_header;
use crate::models::{AuthConfig, CatalogResponse, Manifest, TagsResponse, BlobInfo};
use super::ApiError;

/// Docker Registry API client
pub struct RegistryClient {
    base_url: String,
    auth: AuthConfig,
    client: reqwest::Client,
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(base_url: String, auth: AuthConfig) -> Result<Self, ApiError> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        // Normalize URL - remove trailing slash
        let base_url = base_url.trim_end_matches('/').to_string();
        
        Ok(Self {
            base_url,
            auth,
            client,
        })
    }
    
    /// Build a request with authentication
    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.request(method, &url);
        
        if let Some(auth_header) = get_auth_header(&self.auth) {
            req = req.header("Authorization", auth_header);
        }
        
        req
    }
    
    /// Check registry availability (ping)
    pub async fn ping(&self) -> Result<(), ApiError> {
        let response = self.request(reqwest::Method::GET, "/v2/")
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        if status == 200 || status == 401 {
            // 401 means registry is available but needs auth
            Ok(())
        } else {
            Err(ApiError::from_status(status, "Registry not available".to_string()))
        }
    }
    
    /// Get repository catalog
    pub async fn get_catalog(&self, page: Option<&str>) -> Result<CatalogResponse, ApiError> {
        let path = match page {
            Some(p) => format!("/v2/_catalog?{}", p),
            None => "/v2/_catalog".to_string(),
        };
        
        let response = self.request(reqwest::Method::GET, &path)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        if status != 200 {
            return Err(ApiError::from_status(status, "Failed to get catalog".to_string()));
        }
        
        // Check for pagination Link header
        let next_page = response
            .headers()
            .get("Link")
            .and_then(|v| v.to_str().ok())
            .and_then(parse_link_header);
        
        let mut catalog: CatalogResponse = response
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;
        
        catalog.next_page = next_page;
        Ok(catalog)
    }
    
    /// Get tags for a repository
    pub async fn get_tags(&self, repo: &str) -> Result<TagsResponse, ApiError> {
        let path = format!("/v2/{}/tags/list", repo);
        
        let response = self.request(reqwest::Method::GET, &path)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        if status != 200 {
            return Err(ApiError::from_status(status, format!("Failed to get tags for {}", repo)));
        }
        
        response
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))
    }
    
    /// Get manifest by reference (tag or digest)
    pub async fn get_manifest(&self, repo: &str, reference: &str) -> Result<(Manifest, String), ApiError> {
        let path = format!("/v2/{}/manifests/{}", repo, reference);
        
        let response = self.request(reqwest::Method::GET, &path)
            .header("Accept", "application/vnd.docker.distribution.manifest.v2+json, application/vnd.oci.image.manifest.v1+json, application/vnd.docker.distribution.manifest.v1+json")
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        if status != 200 {
            return Err(ApiError::from_status(status, format!("Failed to get manifest for {}:{}", repo, reference)));
        }
        
        // Get digest from header
        let digest = response
            .headers()
            .get("Docker-Content-Digest")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_default();
        
        let manifest: Manifest = response
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;
        
        Ok((manifest, digest))
    }
    
    /// Delete manifest by digest
    pub async fn delete_manifest(&self, repo: &str, digest: &str) -> Result<(), ApiError> {
        let path = format!("/v2/{}/manifests/{}", repo, digest);
        
        let response = self.request(reqwest::Method::DELETE, &path)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        if status != 202 && status != 200 {
            return Err(ApiError::from_status(status, format!("Failed to delete manifest {}", digest)));
        }
        
        Ok(())
    }
    
    /// Get blob info (HEAD request)
    pub async fn head_blob(&self, repo: &str, digest: &str) -> Result<BlobInfo, ApiError> {
        let path = format!("/v2/{}/blobs/{}", repo, digest);
        
        let response = self.request(reqwest::Method::HEAD, &path)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        if status != 200 {
            return Err(ApiError::from_status(status, format!("Failed to get blob {}", digest)));
        }
        
        let size = response
            .headers()
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        
        let media_type = response
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        
        Ok(BlobInfo {
            digest: digest.to_string(),
            size,
            media_type,
        })
    }
}

/// Parse Link header for pagination
fn parse_link_header(header: &str) -> Option<String> {
    // Format: </v2/_catalog?n=100&last=repo>; rel="next"
    for part in header.split(',') {
        if part.contains("rel=\"next\"") || part.contains("rel=next") {
            let url_part = part.split(';').next()?;
            let url = url_part.trim().trim_start_matches('<').trim_end_matches('>');
            // Extract query string
            return url.split('?').nth(1).map(|s| s.to_string());
        }
    }
    None
}
