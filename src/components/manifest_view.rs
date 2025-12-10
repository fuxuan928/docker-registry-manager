//! Manifest view component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::api::RegistryClient;
use crate::models::{Manifest, HistoryEntry};
use crate::utils::{format_size, sorted_history_chronologically};

/// Manifest details view component
#[component]
pub fn ManifestView() -> Element {
    let app_state = use_context::<AppState>();
    let selected_tag = app_state.selected_tag.read().clone();
    
    let mut manifest = use_signal(|| None::<Manifest>);
    let mut digest = use_signal(String::new);
    let mut raw_json = use_signal(String::new);
    let mut show_raw = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    
    // Fetch manifest when tag changes
    let _fetch = use_resource(move || {
        let registry_id = app_state.selected_registry.read().clone();
        let repo = app_state.selected_repo.read().clone();
        let tag = app_state.selected_tag.read().clone();
        async move {
            if let (Some(id), Some(repo_name), Some(tag_name)) = (registry_id, repo, tag) {
                if let Some(registry) = app_state.get_registry(&id) {
                    loading.set(true);
                    error.set(None);
                    
                    match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                        Ok(client) => {
                            match client.get_manifest(&repo_name, &tag_name).await {
                                Ok((m, d)) => {
                                    // Store raw JSON
                                    if let Ok(json) = serde_json::to_string_pretty(&m) {
                                        raw_json.set(json);
                                    }
                                    digest.set(d);
                                    manifest.set(Some(m));
                                    error.set(None);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to fetch manifest: {}", e)));
                                    manifest.set(None);
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to create client: {}", e)));
                            manifest.set(None);
                        }
                    }
                    
                    loading.set(false);
                }
            } else {
                manifest.set(None);
                digest.set(String::new());
                raw_json.set(String::new());
                error.set(None);
            }
        }
    });
    
    
    rsx! {
        div {
            class: "manifest-view",
            
            div {
                class: "panel-header",
                h3 { "Manifest Details" }
            }
            
            if selected_tag.is_none() {
                p {
                    class: "empty-message",
                    "Select a tag to view manifest details"
                }
            } else if loading() {
                p { class: "loading", "Loading manifest..." }
            } else if let Some(err) = error() {
                p { class: "error", "{err}" }
            } else if let Some(m) = manifest() {
                div {
                    class: "manifest-content",
                    
                    // Basic info
                    div {
                        class: "manifest-section",
                        h4 { "Overview" }
                        dl {
                            dt { "Tag" }
                            dd { "{selected_tag.clone().unwrap_or_default()}" }
                            dt { "Digest" }
                            dd { 
                                class: "digest-value",
                                "{digest}" 
                            }
                            dt { "Media Type" }
                            dd { "{m.media_type()}" }
                            dt { "Total Size" }
                            dd { "{format_size(m.total_size())}" }
                        }
                    }
                    
                    // Layers
                    div {
                        class: "manifest-section",
                        h4 { "Layers ({m.layers().len()})" }
                        div {
                            class: "layers-list",
                            for (i, layer) in m.layers().iter().enumerate() {
                                div {
                                    key: "{layer.digest}",
                                    class: "layer-item",
                                    span { class: "layer-index", "{i + 1}" }
                                    span { class: "layer-digest", title: "{layer.digest}", "{truncate_digest(&layer.digest)}" }
                                    span { class: "layer-size", "{format_size(layer.size)}" }
                                }
                            }
                        }
                    }
                    
                    // Raw JSON toggle
                    div {
                        class: "manifest-section",
                        button {
                            class: "secondary",
                            onclick: move |_| show_raw.set(!show_raw()),
                            if show_raw() { "Hide Raw JSON" } else { "Show Raw JSON" }
                        }
                        
                        if show_raw() {
                            pre {
                                class: "raw-json",
                                "{raw_json}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Truncate digest for display
fn truncate_digest(digest: &str) -> String {
    if digest.len() > 24 {
        format!("{}...", &digest[..24])
    } else {
        digest.to_string()
    }
}

/// History view component
#[component]
pub fn HistoryView(history: Vec<HistoryEntry>) -> Element {
    let sorted = sorted_history_chronologically(&history);
    
    rsx! {
        div {
            class: "history-view",
            h4 { "Build History" }
            
            if sorted.is_empty() {
                p { class: "empty-message", "No history available" }
            } else {
                div {
                    class: "history-list",
                    for (i, entry) in sorted.iter().enumerate() {
                        div {
                            key: "{i}",
                            class: "history-item",
                            
                            if let Some(created) = &entry.created {
                                span { class: "history-time", "{created}" }
                            }
                            
                            if let Some(cmd) = &entry.created_by {
                                pre { class: "history-command", "{cmd}" }
                            }
                            
                            if entry.empty_layer == Some(true) {
                                span { class: "empty-layer-badge", "empty layer" }
                            }
                        }
                    }
                }
            }
        }
    }
}
