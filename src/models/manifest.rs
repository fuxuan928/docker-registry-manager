//! Docker manifest and related models

use serde::{Deserialize, Serialize};

/// Catalog response from /v2/_catalog
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogResponse {
    pub repositories: Vec<String>,
    #[serde(skip)]
    pub next_page: Option<String>,
}

/// Tags response from /v2/<name>/tags/list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResponse {
    pub name: String,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// Docker manifest - supports multiple schema versions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Manifest {
    V2(ManifestV2),
    OCI(OciManifest),
    V1(ManifestV1),
}

/// Docker manifest schema version 2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManifestV2 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub config: Descriptor,
    pub layers: Vec<Descriptor>,
}

/// OCI manifest format
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OciManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,
    #[serde(rename = "mediaType", default)]
    pub media_type: Option<String>,
    pub config: Descriptor,
    pub layers: Vec<Descriptor>,
}

/// Docker manifest schema version 1 (legacy)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManifestV1 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,
    pub name: String,
    pub tag: String,
    pub architecture: String,
    #[serde(rename = "fsLayers", default)]
    pub fs_layers: Vec<FsLayer>,
    #[serde(default)]
    pub history: Vec<V1History>,
}

/// File system layer in V1 manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FsLayer {
    #[serde(rename = "blobSum")]
    pub blob_sum: String,
}

/// History entry in V1 manifest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1History {
    #[serde(rename = "v1Compatibility")]
    pub v1_compatibility: String,
}

/// Content descriptor for blobs and configs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Descriptor {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: u64,
    pub digest: String,
}

/// Image configuration from config blob
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageConfig {
    pub architecture: String,
    pub os: String,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub history: Option<Vec<HistoryEntry>>,
}

/// History entry from image config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub empty_layer: Option<bool>,
}

/// Blob information from HEAD request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobInfo {
    pub digest: String,
    pub size: u64,
    #[serde(default)]
    pub media_type: Option<String>,
}

/// Tag information with digest and size
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagInfo {
    pub name: String,
    pub digest: String,
    pub size: u64,
}

impl Manifest {
    /// Get the media type of the manifest
    pub fn media_type(&self) -> &str {
        match self {
            Manifest::V2(m) => &m.media_type,
            Manifest::OCI(m) => m.media_type.as_deref().unwrap_or("application/vnd.oci.image.manifest.v1+json"),
            Manifest::V1(_) => "application/vnd.docker.distribution.manifest.v1+json",
        }
    }

    /// Get the layers from the manifest
    pub fn layers(&self) -> Vec<&Descriptor> {
        match self {
            Manifest::V2(m) => m.layers.iter().collect(),
            Manifest::OCI(m) => m.layers.iter().collect(),
            Manifest::V1(_) => vec![],
        }
    }

    /// Calculate total size from layers
    pub fn total_size(&self) -> u64 {
        self.layers().iter().map(|l| l.size).sum()
    }
}
