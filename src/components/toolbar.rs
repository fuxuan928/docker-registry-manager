//! Toolbar component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::Theme;

/// Application toolbar component
#[component]
pub fn Toolbar(show_settings: Signal<bool>) -> Element {
    let mut app_state = use_context::<AppState>();
    let theme = app_state.theme;
    
    rsx! {
        header {
            class: "toolbar",
            
            h1 { "Docker Registry Manager" }
            
            div {
                class: "toolbar-actions",
                
                // Refresh button
                button {
                    class: "btn-icon",
                    title: "Refresh",
                    onclick: move |_| {
                        // TODO: Trigger refresh
                    },
                    "🔄"
                }
                
                // Theme toggle
                button {
                    class: "btn-icon",
                    title: "Toggle theme",
                    onclick: move |_| {
                        let new_theme = match theme() {
                            Theme::Light => Theme::Dark,
                            Theme::Dark => Theme::System,
                            Theme::System => Theme::Light,
                        };
                        app_state.set_theme(new_theme);
                    },
                    match theme() {
                        Theme::Light => "☀️",
                        Theme::Dark => "🌙",
                        Theme::System => "💻",
                    }
                }
                
                // Settings toggle
                button {
                    class: if show_settings() { "btn-icon active" } else { "btn-icon" },
                    title: "Settings",
                    onclick: move |_| show_settings.set(!show_settings()),
                    "⚙️"
                }
            }
        }
    }
}
