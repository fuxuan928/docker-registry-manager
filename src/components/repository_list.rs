//! Repository list component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::api::RegistryClient;
use crate::models::AuthConfig;
use crate::components::delete_dialog::{DeleteRepositoryDialog, DeletionResult};
use crate::utils::{filter_strings_owned, sorted_alphabetically};

/// Repository list component
#[component]
pub fn RepositoryList() -> Element {
    let mut app_state = use_context::<AppState>();
    let selected_registry_id = app_state.selected_registry.read().clone();
    let selected_repo = app_state.selected_repo.read().clone();
    
    let mut search = use_signal(String::new);
    let mut repositories = use_signal(Vec::<String>::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut delete_status = use_signal(|| None::<String>);
    
    // Delete dialog state
    let mut show_delete_dialog = use_signal(|| false);
    let mut delete_repo_name = use_signal(String::new);
    let mut delete_tags = use_signal(Vec::<String>::new);
    let mut delete_registry_url = use_signal(String::new);
    let mut delete_registry_auth = use_signal(|| AuthConfig::Anonymous);
    
    // Get the selected registry config
    let selected_registry = selected_registry_id.as_ref()
        .and_then(|id| app_state.get_registry(id));
    
    // Fetch repositories when registry changes
    let _fetch = use_resource(move || {
        let registry_id = app_state.selected_registry.read().clone();
        async move {
            if let Some(id) = registry_id {
                if let Some(registry) = app_state.get_registry(&id) {
                    loading.set(true);
                    error.set(None);
                    
                    match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                        Ok(client) => {
                            match client.get_catalog(None).await {
                                Ok(catalog) => {
                                    repositories.set(catalog.repositories);
                                    error.set(None);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to fetch repositories: {}", e)));
                                    repositories.set(Vec::new());
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to create client: {}", e)));
                            repositories.set(Vec::new());
                        }
                    }
                    
                    loading.set(false);
                }
            } else {
                repositories.set(Vec::new());
                error.set(None);
            }
        }
    });
    
    // Filter and sort repositories
    let filtered = use_memo(move || {
        let repos = repositories.read();
        let search_term = search();
        let filtered = filter_strings_owned(&repos, &search_term);
        sorted_alphabetically(&filtered)
    });
    
    // Manual refresh function
    let refresh = move |_| {
        if let Some(id) = app_state.selected_registry.read().clone() {
            if let Some(registry) = app_state.get_registry(&id) {
                loading.set(true);
                error.set(None);
                
                spawn(async move {
                    match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                        Ok(client) => {
                            match client.get_catalog(None).await {
                                Ok(catalog) => {
                                    repositories.set(catalog.repositories);
                                    error.set(None);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to fetch: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Client error: {}", e)));
                        }
                    }
                    loading.set(false);
                });
            }
        }
    };
    
    // Initiate delete for a repository
    let mut initiate_delete = move |repo: String| {
        if let Some(id) = app_state.selected_registry.read().clone() {
            if let Some(registry) = app_state.get_registry(&id) {
                let repo_clone = repo.clone();
                let url = registry.url.clone();
                let auth = registry.auth.clone();
                
                // Store registry info for dialog
                delete_repo_name.set(repo);
                delete_registry_url.set(url.clone());
                delete_registry_auth.set(auth.clone());
                
                // Fetch tags for the repository
                spawn(async move {
                    match RegistryClient::new(url, auth) {
                        Ok(client) => {
                            match client.get_tags(&repo_clone).await {
                                Ok(tags_response) => {
                                    let tags = tags_response.tags.unwrap_or_default();
                                    delete_tags.set(tags);
                                    show_delete_dialog.set(true);
                                }
                                Err(e) => {
                                    delete_status.set(Some(format!("Failed to fetch tags: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            delete_status.set(Some(format!("Client error: {}", e)));
                        }
                    }
                });
            }
        }
    };
    
    // Close delete dialog
    let mut close_dialog = move || {
        show_delete_dialog.set(false);
        delete_repo_name.set(String::new());
        delete_tags.set(Vec::new());
        delete_registry_url.set(String::new());
        delete_registry_auth.set(AuthConfig::Anonymous);
    };
    
    rsx! {
        div {
            class: "repository-list",
            
            div {
                class: "panel-header",
                h3 { "Repositories" }
                
                if selected_registry.is_some() {
                    button {
                        class: "btn-icon small",
                        title: "Refresh",
                        onclick: refresh,
                        "🔄"
                    }
                }
            }
            
            // Delete status message
            if let Some(status) = delete_status() {
                div {
                    class: "status-message",
                    "{status}"
                    button {
                        class: "btn-icon small",
                        onclick: move |_| delete_status.set(None),
                        "×"
                    }
                }
            }
            
            // Delete dialog
            if show_delete_dialog() && !delete_repo_name().is_empty() {
                DeleteRepositoryDialog {
                    repo_name: delete_repo_name(),
                    tags: delete_tags(),
                    registry_url: delete_registry_url(),
                    registry_auth: delete_registry_auth(),
                    on_confirm: move |result: DeletionResult| {
                        let deleted_repo = delete_repo_name();
                        close_dialog();
                        
                        // Show result status
                        if result.failed == 0 {
                            delete_status.set(Some(format!("Deleted {} tags successfully", result.deleted)));
                        } else {
                            delete_status.set(Some(format!("Deleted {}, {} failed", result.deleted, result.failed)));
                        }
                        
                        // Refresh repository list
                        if let Some(id) = app_state.selected_registry.read().clone() {
                            if let Some(registry) = app_state.get_registry(&id) {
                                spawn(async move {
                                    if let Ok(client) = RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                                        if let Ok(catalog) = client.get_catalog(None).await {
                                            repositories.set(catalog.repositories);
                                        }
                                    }
                                });
                            }
                        }
                        
                        // Clear selection if deleted repo was selected
                        if app_state.selected_repo.read().as_ref() == Some(&deleted_repo) {
                            app_state.select_repo(None);
                        }
                    },
                    on_cancel: move |_| {
                        close_dialog();
                    },
                }
            }
            
            if selected_registry.is_none() {
                p {
                    class: "empty-message",
                    "Select a registry to view repositories"
                }
            } else {
                // Search input
                div {
                    class: "search-box",
                    input {
                        r#type: "text",
                        placeholder: "Search repositories...",
                        value: "{search}",
                        oninput: move |e| search.set(e.value()),
                    }
                }
                
                if loading() {
                    p { class: "loading", "Loading..." }
                } else if let Some(err) = error() {
                    div {
                        class: "error-box",
                        p { class: "error", "{err}" }
                        button {
                            class: "secondary small",
                            onclick: refresh,
                            "Retry"
                        }
                    }
                } else if filtered().is_empty() {
                    p {
                        class: "empty-message",
                        if search().is_empty() {
                            "No repositories found"
                        } else {
                            "No matching repositories"
                        }
                    }
                } else {
                    div {
                        class: "list",
                        for repo in filtered() {
                            RepositoryItem {
                                repo: repo.clone(),
                                is_selected: selected_repo.as_ref() == Some(&repo),
                                on_select: move |name: String| app_state.select_repo(Some(name)),
                                on_delete: move |name: String| initiate_delete(name),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Single repository item
#[component]
fn RepositoryItem(
    repo: String,
    is_selected: bool,
    on_select: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let repo_select = repo.clone();
    let repo_delete = repo.clone();
    
    rsx! {
        div {
            class: if is_selected { "list-item repo-item selected" } else { "list-item repo-item" },
            onclick: move |_| on_select.call(repo_select.clone()),
            
            span { class: "repo-name", "{repo}" }
            
            button {
                class: "btn-icon small danger",
                title: "Delete repository",
                onclick: move |e| {
                    e.stop_propagation();
                    on_delete.call(repo_delete.clone());
                },
                "🗑️"
            }
        }
    }
}
