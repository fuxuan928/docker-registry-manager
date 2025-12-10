//! Application state management

use dioxus::prelude::*;
use crate::models::{CacheConfig, RegistryConfig, Theme};
use crate::storage::get_storage;

/// Global application state - uses Copy-able Signal wrappers
#[derive(Clone, Copy)]
pub struct AppState {
    /// List of configured registries
    pub registries: Signal<Vec<RegistryConfig>>,
    /// Currently selected registry ID
    pub selected_registry: Signal<Option<String>>,
    /// Currently selected repository
    pub selected_repo: Signal<Option<String>>,
    /// Currently selected tag
    pub selected_tag: Signal<Option<String>>,
    /// Current theme
    pub theme: Signal<Theme>,
    /// Cache configuration
    pub cache_config: Signal<CacheConfig>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            registries: Signal::new(Vec::new()),
            selected_registry: Signal::new(None),
            selected_repo: Signal::new(None),
            selected_tag: Signal::new(None),
            theme: Signal::new(Theme::default()),
            cache_config: Signal::new(CacheConfig::default()),
        }
    }
    
    /// Persist registries to storage
    fn persist_registries(&self) {
        let registries: Vec<RegistryConfig> = self.registries.read().clone();
        let storage = get_storage();
        if let Err(e) = storage.save_registries(&registries) {
            eprintln!("[ERROR] Failed to save registries: {:?}", e);
        }
    }
    
    /// Add a new registry and persist
    pub fn add_registry(&mut self, registry: RegistryConfig) {
        self.registries.write().push(registry);
        self.persist_registries();
    }
    
    /// Update an existing registry and persist
    pub fn update_registry(&mut self, id: &str, updated: RegistryConfig) {
        {
            let mut registries = self.registries.write();
            if let Some(reg) = registries.iter_mut().find(|r| r.id == id) {
                *reg = updated;
            }
        }
        self.persist_registries();
    }
    
    /// Delete a registry by ID and persist
    pub fn delete_registry(&mut self, id: &str) {
        self.registries.write().retain(|r| r.id != id);
        if self.selected_registry.read().as_ref() == Some(&id.to_string()) {
            *self.selected_registry.write() = None;
            *self.selected_repo.write() = None;
            *self.selected_tag.write() = None;
        }
        self.persist_registries();
    }
    
    /// Get registry by ID
    pub fn get_registry(&self, id: &str) -> Option<RegistryConfig> {
        self.registries.read().iter().find(|r| r.id == id).cloned()
    }
    
    /// Select a registry
    pub fn select_registry(&mut self, id: Option<String>) {
        *self.selected_registry.write() = id;
        *self.selected_repo.write() = None;
        *self.selected_tag.write() = None;
    }
    
    /// Select a repository
    pub fn select_repo(&mut self, repo: Option<String>) {
        *self.selected_repo.write() = repo;
        *self.selected_tag.write() = None;
    }
    
    /// Select a tag
    pub fn select_tag(&mut self, tag: Option<String>) {
        *self.selected_tag.write() = tag;
    }
    
    /// Set theme and persist
    pub fn set_theme(&mut self, new_theme: Theme) {
        *self.theme.write() = new_theme.clone();
        let storage = get_storage();
        let _ = storage.save_theme(&new_theme);
    }
    
    /// Set cache config and persist
    pub fn set_cache_config(&mut self, config: CacheConfig) {
        *self.cache_config.write() = config.clone();
        let storage = get_storage();
        let _ = storage.save_cache_config(&config);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
