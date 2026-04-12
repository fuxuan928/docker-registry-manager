//! Tag list component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::api::RegistryClient;
use crate::models::TagInfo;
use crate::utils::format_size;

fn tag_checkbox_class() -> &'static str {
    "tag-item-checkbox"
}

/// Tag list component
#[component]
pub fn TagList() -> Element {
    let mut app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let selected_registry_id = app_state.selected_registry.read().clone();
    let selected_repo = app_state.selected_repo.read().clone();
    let selected_tag = app_state.selected_tag.read().clone();
    let panel_class = if selected_repo.is_some() {
        "tag-list workspace-panel is-primary"
    } else {
        "tag-list workspace-panel is-secondary"
    };
    
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

    use_effect(move || {
        let _ = app_state.selected_registry.read().clone();
        let _ = app_state.selected_repo.read().clone();
        selected_tags.set(Vec::new());
        show_delete_confirm.set(false);
    });
    
    // Fetch tags when repo changes
    let _fetch = use_resource(move || {
        let _ = (app_state.refresh_tick)();
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
                                    if app_state.selected_registry.read().as_ref() == Some(&id)
                                        && app_state.selected_repo.read().as_ref() == Some(&repo_name)
                                    {
                                        tags.set(tag_infos);
                                        error.set(None);
                                    }
                                }
                                Err(e) => {
                                    if app_state.selected_registry.read().as_ref() == Some(&id)
                                        && app_state.selected_repo.read().as_ref() == Some(&repo_name)
                                    {
                                        error.set(Some(strings.failed_to_fetch_tags(&e.to_string())));
                                        tags.set(Vec::new());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if app_state.selected_registry.read().as_ref() == Some(&id)
                                && app_state.selected_repo.read().as_ref() == Some(&repo_name)
                            {
                                error.set(Some(strings.failed_to_create_client(&e.to_string())));
                                tags.set(Vec::new());
                            }
                        }
                    }
                    
                    if app_state.selected_registry.read().as_ref() == Some(&id)
                        && app_state.selected_repo.read().as_ref() == Some(&repo_name)
                    {
                        loading.set(false);
                    }
                }
            } else {
                tags.set(Vec::new());
                error.set(None);
                loading.set(false);
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
    
    rsx! {
        div {
            class: panel_class,

            div {
                class: "workspace-panel-header",
                div {
                    class: "workspace-panel-title-group",
                    h3 { "{strings.tags()}" }
                    p { "{strings.tags_subtitle()}" }
                }

                if selected_repo.is_some() && selected_registry.is_some() {
                    div {
                        class: "workspace-panel-actions",
                        button {
                            class: "btn-icon small",
                            title: "{strings.refresh()}",
                            onclick: move |_| app_state.request_refresh(),
                            "🔄"
                        }
                    }
                }
            }

            div {
                class: "workspace-panel-body",

                if selected_repo.is_none() {
                    p {
                        class: "empty-message",
                        "{strings.select_repository_to_view_tags()}"
                    }
                } else {
                    div {
                        class: "workspace-panel-actions",
                        div {
                            class: "search-box",
                            input {
                                r#type: "text",
                                placeholder: "{strings.search_tags()}" ,
                                value: "{search}",
                                oninput: move |e| search.set(e.value()),
                            }
                        }

                        if !selected_tags().is_empty() {
                            div {
                                class: "batch-actions",
                                span { "{strings.selected_count(selected_tags().len())}" }
                                button {
                                    class: "danger small",
                                    onclick: move |_| {
                                        show_delete_confirm.set(true);
                                    },
                                    "{strings.delete_selected()}"
                                }
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
                                        delete_status.set(Some(strings.deleting().to_string()));

                                        spawn(async move {
                                            match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                                                Ok(client) => {
                                                    let mut deleted = 0;
                                                    let mut deleted_tag_names = Vec::new();
                                                    let mut errors = Vec::new();

                                                    for tag_name in &tags_to_del {
                                                        // First get the manifest to get the digest
                                                        match client.get_manifest(&repo_name, tag_name).await {
                                                            Ok((_, digest)) => {
                                                                if !digest.is_empty() {
                                                                    match client.delete_manifest(&repo_name, &digest).await {
                                                                        Ok(_) => {
                                                                            deleted += 1;
                                                                            deleted_tag_names.push(tag_name.clone());
                                                                        }
                                                                        Err(e) => errors.push(strings.tag_error_entry(tag_name, &e.to_string())),
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => errors.push(strings.tag_error_entry(tag_name, &e.to_string())),
                                                        }
                                                    }

                                                    if errors.is_empty() {
                                                        delete_status.set(Some(strings.deleted_tags(deleted)));
                                                    } else {
                                                        delete_status.set(Some(strings.deleted_tags_with_errors(deleted, errors.len())));
                                                    }

                                                    if app_state.selected_tag.read().as_ref().is_some_and(|tag| deleted_tag_names.contains(tag)) {
                                                        app_state.select_tag(None);
                                                    }

                                                    // Refresh tags list
                                                    if app_state.selected_registry.read().as_ref() == Some(&id)
                                                        && app_state.selected_repo.read().as_ref() == Some(&repo_name)
                                                    {
                                                        app_state.request_refresh();
                                                    }
                                                }
                                                Err(e) => {
                                                    delete_status.set(Some(strings.error_with_details(&e.to_string())));
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
                        p { class: "loading", "{strings.loading()}" }
                    } else if let Some(err) = error() {
                        div {
                            class: "error-box",
                            p { class: "error", "{err}" }
                            button {
                                class: "secondary small",
                                onclick: move |_| app_state.request_refresh(),
                                "{strings.retry()}"
                            }
                        }
                    } else if filtered().is_empty() {
                        p {
                            class: "empty-message",
                            if search().is_empty() {
                                "{strings.no_tags_found()}"
                            } else {
                                "{strings.no_matching_tags()}"
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

            input {
                class: tag_checkbox_class(),
                r#type: "checkbox",
                checked: is_checked,
                onclick: move |e| {
                    e.stop_propagation();
                    on_toggle.call(name_toggle.clone());
                },
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
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let count = tags_to_delete.len();
    
    rsx! {
        div {
            class: "modal-overlay",
            
            div {
                class: "modal delete-dialog",
                onclick: move |e| e.stop_propagation(),
                
                h3 { "{strings.delete_tags()}" }
                
                div {
                    class: "delete-confirm",
                    p { "{strings.delete_tags_confirm(count)}" }
                    
                    div {
                        class: "tag-list-preview",
                        for tag in tags_to_delete.iter().take(10) {
                            span { class: "tag-badge", "{tag}" }
                        }
                        if count > 10 {
                            span { class: "more-tags", "{strings.more_items(count - 10)}" }
                        }
                    }

                    p { class: "warning", "{strings.action_cannot_be_undone()}" }
                    
                    div {
                        class: "form-actions",
                        button {
                            class: "secondary",
                            onclick: move |_| on_cancel.call(()),
                            "{strings.cancel()}"
                        }
                        button {
                            class: "danger",
                            onclick: move |_| on_confirm.call(()),
                            "{strings.delete_action()}"
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tag_checkbox_class;

    #[test]
    fn tag_checkbox_uses_compact_dedicated_class() {
        assert_eq!(tag_checkbox_class(), "tag-item-checkbox");
    }
}
