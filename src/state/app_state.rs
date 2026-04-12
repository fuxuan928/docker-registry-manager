//! Application state management

use crate::i18n::{effective_locale, strings_for_locale, Strings};
use crate::models::{CacheConfig, ConnectionStatus, Locale, RegistryConfig, Theme};
use crate::storage::get_storage;
use dioxus::prelude::*;

fn next_refresh_tick(current: u64) -> u64 {
    current.wrapping_add(1)
}

fn apply_registry_status(registries: &mut [RegistryConfig], id: &str, status: ConnectionStatus) {
    if let Some(registry) = registries.iter_mut().find(|registry| registry.id == id) {
        registry.status = status;
    }
}

pub fn theme_class(theme: &Theme) -> &'static str {
    match theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
        Theme::System => "system",
    }
}

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
    /// Current locale preference
    pub locale: Signal<Locale>,
    /// Cache configuration
    pub cache_config: Signal<CacheConfig>,
    /// Global refresh tick
    pub refresh_tick: Signal<u64>,
}

impl AppState {
    pub fn load_from_storage() -> Self {
        let mut state = Self::new();
        state.reload_from_storage();
        state
    }

    pub fn reload_from_storage(&mut self) {
        let storage = get_storage();

        if let Ok(theme) = storage.load_theme() {
            *self.theme.write() = theme;
        }

        if let Ok(locale) = storage.load_locale() {
            *self.locale.write() = locale;
        }

        if let Ok(cache_config) = storage.load_cache_config() {
            *self.cache_config.write() = cache_config;
        }

        if let Ok(registries) = storage.load_registries() {
            *self.registries.write() = registries;
        }
    }

    /// Create a new application state
    pub fn new() -> Self {
        Self {
            registries: Signal::new(Vec::new()),
            selected_registry: Signal::new(None),
            selected_repo: Signal::new(None),
            selected_tag: Signal::new(None),
            theme: Signal::new(Theme::default()),
            locale: Signal::new(Locale::default()),
            cache_config: Signal::new(CacheConfig::default()),
            refresh_tick: Signal::new(0),
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

    pub fn set_registry_status(&mut self, id: &str, status: ConnectionStatus) {
        apply_registry_status(&mut self.registries.write(), id, status);
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

    /// Set locale and persist
    pub fn set_locale(&mut self, new_locale: Locale) {
        *self.locale.write() = new_locale;
        let storage = get_storage();
        let _ = storage.save_locale(&new_locale);
    }

    pub fn resolved_locale(&self) -> Locale {
        effective_locale((self.locale)())
    }

    pub fn strings(&self) -> &'static dyn Strings {
        strings_for_locale((self.locale)())
    }

    /// Set cache config and persist
    pub fn set_cache_config(&mut self, config: CacheConfig) {
        *self.cache_config.write() = config.clone();
        let storage = get_storage();
        let _ = storage.save_cache_config(&config);
    }

    /// Request a global refresh for data-bound views
    pub fn request_refresh(&mut self) {
        let next_tick = next_refresh_tick((self.refresh_tick)());
        self.refresh_tick.set(next_tick);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_refresh_tick_increments_and_wraps() {
        assert_eq!(next_refresh_tick(0), 1);
        assert_eq!(next_refresh_tick(u64::MAX), 0);
    }

    #[test]
    fn theme_class_maps_all_theme_variants() {
        assert_eq!(theme_class(&Theme::Light), "light");
        assert_eq!(theme_class(&Theme::Dark), "dark");
        assert_eq!(theme_class(&Theme::System), "system");
    }

    #[test]
    fn set_registry_status_updates_matching_registry_only() {
        let mut registries = vec![
            RegistryConfig {
                id: "a".to_string(),
                name: "A".to_string(),
                url: "https://a.example.com".to_string(),
                auth: crate::models::AuthConfig::Anonymous,
                status: ConnectionStatus::Unknown,
            },
            RegistryConfig {
                id: "b".to_string(),
                name: "B".to_string(),
                url: "https://b.example.com".to_string(),
                auth: crate::models::AuthConfig::Anonymous,
                status: ConnectionStatus::Unknown,
            },
        ];

        apply_registry_status(&mut registries, "b", ConnectionStatus::Connected);

        assert!(matches!(registries[0].status, ConnectionStatus::Unknown));
        assert!(matches!(registries[1].status, ConnectionStatus::Connected));
    }
}
