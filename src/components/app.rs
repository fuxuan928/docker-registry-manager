//! Main App component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::Theme;
use crate::storage::get_storage;
use super::{RegistryList, RepositoryList, TagList, ManifestView, Toolbar, Settings};

/// Main application component
#[component]
pub fn App() -> Element {
    // Initialize app state and provide via context
    let app_state = use_context_provider(|| {
        let mut state = AppState::new();
        
        // Load saved data from storage
        let storage = get_storage();
        
        if let Ok(registries) = storage.load_registries() {
            *state.registries.write() = registries;
        }
        
        if let Ok(theme) = storage.load_theme() {
            *state.theme.write() = theme;
        }
        
        if let Ok(cache_config) = storage.load_cache_config() {
            *state.cache_config.write() = cache_config;
        }
        
        state
    });
    
    let theme = app_state.theme;
    let show_settings = use_signal(|| false);
    
    // Get theme class
    let theme_class = match theme() {
        Theme::Light => "light",
        Theme::Dark => "dark",
        Theme::System => "system",
    };
    
    rsx! {
        div {
            class: "app-layout",
            "data-theme": theme_class,
            
            // Left sidebar - Registry list
                aside {
                    class: "sidebar",
                    RegistryList {}
                }
                
                // Main content area
                main {
                    class: "main-content",
                    
                    // Toolbar
                    Toolbar { show_settings }
                    
                    // Content area
                    section {
                        class: "content",
                        
                        if show_settings() {
                            Settings {}
                        } else {
                            div {
                                class: "content-panels",
                                
                                // Repository list panel
                                div {
                                    class: "panel repositories-panel",
                                    RepositoryList {}
                                }
                                
                                // Tags panel
                                div {
                                    class: "panel tags-panel",
                                    TagList {}
                                }
                                
                                // Manifest details panel
                                div {
                                    class: "panel manifest-panel",
                                    ManifestView {}
                                }
                            }
                        }
                    }
                }
        }
    }
}
