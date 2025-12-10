//! Tag list component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::api::RegistryClient;
use crate::models::TagInfo;
use crate::utils::format_size;

/// Tag list component
#[component]
pub fn TagList() -> Element {
    let mut app_state = use_context::<AppState>();
    let selected_registry_id = app_state.selected_registry.read().clone();
    let selected_repo = app_state.selected_repo.read().clone();
    let selected_tag = app_state.selected_tag.read().clone();
    
    let mut search = use_signal(String::new);
    let mut tags = use_signal(Vec::<TagInfo>::new);
    let mut selected_tags = use_signal(Vec::<String>::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut delete_status = use_signal(|| None::<String>);
    let mut show_delete_confirm = use_signal(|| false);
    
    // Get the selected registry config
    let selected_registry = selected_registry_id.as_ref()
        .and_then(|id| app_state.get_registry(id));
    
    // Fetch tags when repo changes
    let _fetch = use_resource(move || {
        let registry_id = app_state.selected_registry.read().clone();
        let repo = app_state.selected_repo.read().clone();
        async move {
            if let (Some(id), Some(repo_name)) = (registry_id, repo) {
                if let Some(registry) = app_state.get_registry(&id) {
                    loading.set(true);
                    error.set(None);
                    
                    match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                        Ok(client) => {
                            match client.get_tags(&repo_name).await {
                                Ok(tags_response) => {
                                    // Convert to TagInfo (without size/digest for now)
                                    let tag_infos: Vec<TagInfo> = tags_response.tags
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(|name| TagInfo {
                                            name,
                                            digest: String::new(),
                                            size: 0,
                                        })
                                        .collect();
                                    tags.set(tag_infos);
                                    error.set(None);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to fetch tags: {}", e)));
                                    tags.set(Vec::new());
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to create client: {}", e)));
                            tags.set(Vec::new());
                        }
                    }
                    
                    loading.set(false);
                }
            } else {
                tags.set(Vec::new());
                error.set(None);
            }
        }
    });
    
    // Filter tags
    let filtered = use_memo(move || {
        let tag_list = tags.read();
        let search_term = search();
        if search_term.is_empty() {
            tag_list.clone()
        } else {
            let search_lower = search_term.to_lowercase();
            tag_list
                .iter()
                .filter(|t| t.name.to_lowercase().contains(&search_lower))
                .cloned()
                .collect()
        }
    });
    
    // Manual refresh function
    let refresh = move |_| {
        if let (Some(id), Some(repo_name)) = (
            app_state.selected_registry.read().clone(),
            app_state.selected_repo.read().clone()
        ) {
            if let Some(registry) = app_state.get_registry(&id) {
                loading.set(true);
                error.set(None);
                
                spawn(async move {
                    match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                        Ok(client) => {
                            match client.get_tags(&repo_name).await {
                                Ok(tags_response) => {
                                    let tag_infos: Vec<TagInfo> = tags_response.tags
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(|name| TagInfo {
                                            name,
                                            digest: String::new(),
                                            size: 0,
                                        })
                                        .collect();
                                    tags.set(tag_infos);
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
    
    rsx! {
        div {
            class: "tag-list",
            
            div {
                class: "panel-header",
                h3 { "Tags" }
                
                if selected_repo.is_some() && selected_registry.is_some() {
                    div {
                        class: "header-actions",
                        button {
                            class: "btn-icon small",
                            title: "Refresh",
                            onclick: refresh,
                            "🔄"
                        }
                    }
                }
            }
            
            if selected_repo.is_none() {
                p {
                    class: "empty-message",
                    "Select a repository to view tags"
                }
            } else {
                // Search input
                div {
                    class: "search-box",
                    input {
                        r#type: "text",
                        placeholder: "Search tags...",
                        value: "{search}",
                        oninput: move |e| search.set(e.value()),
                    }
                }
                
                // Batch actions
                if !selected_tags().is_empty() {
                    div {
                        class: "batch-actions",
                        span { "{selected_tags().len()} selected" }
                        button {
                            class: "danger small",
                            onclick: move |_| {
                                show_delete_confirm.set(true);
                            },
                            "Delete Selected"
                        }
                    }
                }
                
                // Delete confirmation dialog
                if show_delete_confirm() {
                    DeleteTagsDialog {
                        tags_to_delete: selected_tags(),
                        on_confirm: move |_| {
                            let tags_to_del = selected_tags();
                            if let (Some(id), Some(repo_name)) = (
                                app_state.selected_registry.read().clone(),
                                app_state.selected_repo.read().clone()
                            ) {
                                if let Some(registry) = app_state.get_registry(&id) {
                                    delete_status.set(Some("Deleting...".to_string()));
                                    
                                    spawn(async move {
                                        match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                                            Ok(client) => {
                                                let mut deleted = 0;
                                                let mut errors = Vec::new();
                                                
                                                for tag_name in &tags_to_del {
                                                    // First get the manifest to get the digest
                                                    match client.get_manifest(&repo_name, tag_name).await {
                                                        Ok((_, digest)) => {
                                                            if !digest.is_empty() {
                                                                match client.delete_manifest(&repo_name, &digest).await {
                                                                    Ok(_) => deleted += 1,
                                                                    Err(e) => errors.push(format!("{}: {}", tag_name, e)),
                                                                }
                                                            }
                                                        }
                                                        Err(e) => errors.push(format!("{}: {}", tag_name, e)),
                                                    }
                                                }
                                                
                                                if errors.is_empty() {
                                                    delete_status.set(Some(format!("Deleted {} tags", deleted)));
                                                } else {
                                                    delete_status.set(Some(format!("Deleted {}, {} errors", deleted, errors.len())));
                                                }
                                                
                                                // Refresh tags list
                                                if let Ok(tags_response) = client.get_tags(&repo_name).await {
                                                    let tag_infos: Vec<TagInfo> = tags_response.tags
                                                        .unwrap_or_default()
                                                        .into_iter()
                                                        .map(|name| TagInfo {
                                                            name,
                                                            digest: String::new(),
                                                            size: 0,
                                                        })
                                                        .collect();
                                                    tags.set(tag_infos);
                                                }
                                            }
                                            Err(e) => {
                                                delete_status.set(Some(format!("Error: {}", e)));
                                            }
                                        }
                                        selected_tags.set(Vec::new());
                                    });
                                }
                            }
                            show_delete_confirm.set(false);
                        },
                        on_cancel: move |_| {
                            show_delete_confirm.set(false);
                        },
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
                            "No tags found"
                        } else {
                            "No matching tags"
                        }
                    }
                } else {
                    div {
                        class: "list",
                        for tag in filtered() {
                            TagItem {
                                tag: tag.clone(),
                                is_selected: selected_tag.as_ref() == Some(&tag.name),
                                is_checked: selected_tags().contains(&tag.name),
                                on_select: move |name: String| app_state.select_tag(Some(name)),
                                on_toggle: move |name: String| {
                                    let mut tags_sel = selected_tags.write();
                                    if tags_sel.contains(&name) {
                                        tags_sel.retain(|t| t != &name);
                                    } else {
                                        tags_sel.push(name);
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Single tag item
#[component]
fn TagItem(
    tag: TagInfo,
    is_selected: bool,
    is_checked: bool,
    on_select: EventHandler<String>,
    on_toggle: EventHandler<String>,
) -> Element {
    let name = tag.name.clone();
    let name_toggle = tag.name.clone();
    let size_str = if tag.size > 0 { format_size(tag.size) } else { String::new() };
    let digest_short = if tag.digest.len() > 19 {
        format!("{}...", &tag.digest[..19])
    } else if tag.digest.is_empty() {
        String::new()
    } else {
        tag.digest.clone()
    };
    
    rsx! {
        div {
            class: if is_selected { "list-item tag-item selected" } else { "list-item tag-item" },
            onclick: move |_| on_select.call(name.clone()),
            
            input {
                r#type: "checkbox",
                checked: is_checked,
                onclick: move |e| {
                    e.stop_propagation();
                    on_toggle.call(name_toggle.clone());
                },
            }
            
            div {
                class: "tag-info",
                span { class: "tag-name", "{tag.name}" }
                if !digest_short.is_empty() {
                    span { class: "tag-digest", "{digest_short}" }
                }
                if !size_str.is_empty() {
                    span { class: "tag-size", "{size_str}" }
                }
            }
        }
    }
}


/// Delete tags confirmation dialog
#[component]
fn DeleteTagsDialog(
    tags_to_delete: Vec<String>,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let count = tags_to_delete.len();
    
    rsx! {
        div {
            class: "modal-overlay",
            
            div {
                class: "modal delete-dialog",
                onclick: move |e| e.stop_propagation(),
                
                h3 { "Delete Tags" }
                
                div {
                    class: "delete-confirm",
                    p {
                        "Are you sure you want to delete "
                        strong { "{count}" }
                        " tag(s)?"
                    }
                    
                    div {
                        class: "tag-list-preview",
                        for tag in tags_to_delete.iter().take(10) {
                            span { class: "tag-badge", "{tag}" }
                        }
                        if count > 10 {
                            span { class: "more-tags", "...and {count - 10} more" }
                        }
                    }
                    
                    p { class: "warning", "⚠️ This action cannot be undone." }
                    
                    div {
                        class: "form-actions",
                        button {
                            class: "secondary",
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                        button {
                            class: "danger",
                            onclick: move |_| on_confirm.call(()),
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
