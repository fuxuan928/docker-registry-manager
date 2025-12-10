//! Size calculation utilities

use crate::models::{Descriptor, Manifest};

/// Calculate total size from a list of layers
pub fn calculate_total_size(layers: &[Descriptor]) -> u64 {
    layers.iter().map(|l| l.size).sum()
}

/// Calculate total size from a manifest
pub fn manifest_total_size(manifest: &Manifest) -> u64 {
    manifest.total_size()
}

/// Format size in human-readable format
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
