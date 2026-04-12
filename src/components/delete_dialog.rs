//! Delete repository dialog component

use dioxus::prelude::*;
use crate::api::RegistryClient;
use crate::models::AuthConfig;
use crate::state::AppState;

fn dialog_state_class(completed: bool, deleting: bool, has_tags: bool) -> &'static str {
    if completed {
        "delete-dialog-state delete-dialog-complete-state"
    } else if deleting {
        "delete-dialog-state delete-dialog-progress-state"
    } else if has_tags {
        "delete-dialog-state delete-dialog-confirm-state"
    } else {
        "delete-dialog-state delete-dialog-info-state"
    }
}

fn normalized_dialog_flags(completed: bool, deleting: bool) -> (bool, bool) {
    if completed {
        (true, false)
    } else {
        (completed, deleting)
    }
}

#[cfg(target_arch = "wasm32")]
fn copy_button_text(copied: bool) -> &'static str {
    if copied { "✓" } else { "📋" }
}

#[cfg(not(target_arch = "wasm32"))]
fn desktop_copy_feedback_after_attempt() -> bool {
    false
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_button_text(_: bool) -> &'static str {
    "📋"
}

/// Deletion result for tracking progress
#[derive(Clone, Debug)]
pub struct DeletionResult {
    pub deleted: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

/// Delete repository confirmation dialog
#[component]
pub fn DeleteRepositoryDialog(
    repo_name: String,
    tags: Vec<String>,
    registry_url: String,
    registry_auth: AuthConfig,
    on_confirm: EventHandler<DeletionResult>,
    on_cancel: EventHandler<()>,
) -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let mut deleting = use_signal(|| false);
    let mut progress = use_signal(|| 0usize);
    let mut completed = use_signal(|| false);
    let mut result = use_signal(|| DeletionResult {
        deleted: 0,
        failed: 0,
        errors: Vec::new(),
    });
    
    let total_tags = tags.len();
    let has_tags = total_tags > 0;
    let (is_completed, is_deleting) = normalized_dialog_flags(completed(), deleting());
    let state_class = dialog_state_class(is_completed, is_deleting, has_tags);
    let tags_clone = tags.clone();
    let url_clone = registry_url.clone();
    let auth_clone = registry_auth.clone();
    let repo_clone = repo_name.clone();
    
    let start_deletion = move |_| {
        deleting.set(true);
        let tags_to_delete = tags_clone.clone();
        let url = url_clone.clone();
        let auth = auth_clone.clone();
        let repo = repo_clone.clone();
        
        spawn(async move {
            let mut deleted = 0usize;
            let mut failed = 0usize;
            let mut errors = Vec::new();
            
            match RegistryClient::new(url, auth) {
                Ok(client) => {
                    for (i, tag) in tags_to_delete.iter().enumerate() {
                        match client.get_manifest(&repo, tag).await {
                            Ok((_, digest)) => {
                                if !digest.is_empty() {
                                    match client.delete_manifest(&repo, &digest).await {
                                        Ok(_) => deleted += 1,
                                        Err(e) => {
                                            failed += 1;
                                            errors.push(strings.tag_error_entry(tag, &e.to_string()));
                                        }
                                    }
                                } else {
                                    failed += 1;
                                    errors.push(strings.tag_error_entry(tag, strings.no_digest_returned()));
                                }
                            }
                            Err(e) => {
                                failed += 1;
                                errors.push(strings.tag_error_entry(tag, &e.to_string()));
                            }
                        }
                        progress.set(i + 1);
                    }
                }
                Err(e) => {
                    errors.push(strings.client_error(&e.to_string()));
                    failed = tags_to_delete.len();
                }
            }
            
            deleting.set(false);
            result.set(DeletionResult { deleted, failed, errors });
            completed.set(true);
        });
    };
    
    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| {
                if !is_deleting {
                    on_cancel.call(());
                }
            },
            
            div {
                class: "modal delete-dialog",
                onclick: move |e| e.stop_propagation(),

                div {
                    class: "dialog-header",
                    h3 { "{strings.delete_repository_title()}" }
                }
                
                if is_completed {
                    // Deletion completed
                    div {
                        class: "dialog-body {state_class}",
                        div {
                            class: "dialog-section delete-summary",
                            p { "{strings.repository_deletion_completed()}" }
                            p { "{strings.deleted_of_tags(result().deleted, total_tags)}" }
                        }
                        
                        if result().failed > 0 {
                            div {
                                class: "dialog-section error-summary",
                                p { class: "error", "{strings.failed_to_delete_tags(result().failed)}" }
                                div {
                                    class: "error-list",
                                    for err in result().errors.iter().take(5) {
                                        p { class: "error-item", "{err}" }
                                    }
                                    if result().errors.len() > 5 {
                                        {
                                            let more = result().errors.len() - 5;
                                            rsx! { p { class: "more-errors", "{strings.more_items(more)}" } }
                                        }
                                    }
                                }
                            }
                        }
                        
                        div {
                            class: "dialog-footer",
                            button {
                                class: "primary",
                                onclick: move |_| on_confirm.call(result()),
                                "{strings.close()}"
                            }
                        }
                    }
                } else if is_deleting {
                    // Deletion in progress
                    div {
                        class: "dialog-body {state_class}",
                        div {
                            class: "dialog-section delete-progress",
                            p { "{strings.deleting_tags()}" }
                            progress {
                                class: "progress-bar",
                                value: "{progress()}",
                                max: "{total_tags.max(1)}",
                            }
                            p { "{progress()} / {total_tags}" }
                        }
                    }
                } else if !has_tags {
                    // No tags to delete
                    EmptyRepositoryInfo {
                        repo_name: repo_name.clone(),
                        on_close: move |_| on_cancel.call(()),
                    }
                } else {
                    // Confirmation dialog
                    div {
                        class: "dialog-body {state_class}",
                        div {
                            class: "dialog-section delete-summary",
                            p {
                                "{strings.delete_repository_confirm(&repo_name)}"
                            }
                             
                            p { class: "warning", "{strings.delete_all_tags_warning(total_tags)}" }
                        }

                        div {
                            class: "dialog-section tag-list-preview",
                            for tag in tags.iter().take(10) {
                                span { class: "tag-badge", "{tag}" }
                            }
                            if total_tags > 10 {
                                span { class: "more-tags", "{strings.more_items(total_tags - 10)}" }
                            }
                        }
                        
                        div {
                            class: "dialog-footer form-actions",
                            button {
                                class: "secondary",
                                onclick: move |_| on_cancel.call(()),
                                "{strings.cancel()}"
                            }
                            button {
                                class: "danger",
                                onclick: start_deletion,
                                "{strings.delete_all_tags()}"
                            }
                        }
                    }
                }
            }
        }
    }
}


/// Component shown when repository has no tags
#[component]
fn EmptyRepositoryInfo(
    repo_name: String,
    on_close: EventHandler<()>,
) -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let mut copied = use_signal(|| false);
    
    let gc_command = "docker exec <registry-container> bin/registry garbage-collect /etc/docker/registry/config.yml --delete-untagged";
    
    let copy_command = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(clipboard) = window.navigator().clipboard() {
                    let _ = clipboard.write_text(gc_command);
                    copied.set(true);
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            copied.set(desktop_copy_feedback_after_attempt());
        }
    };
    
    rsx! {
        div {
            class: "dialog-body delete-dialog-state delete-dialog-info-state",
            div {
                class: "dialog-section empty-repository-info",
                p { "{strings.repository_has_no_tags(&repo_name)}" }
            }
            
            div {
                class: "dialog-section info-box empty-repository-info",
                p { "{strings.empty_repository_hint()}" }
                p { "{strings.api_repository_delete_not_supported()}" }
                
                div {
                    class: "gc-section",
                    p { 
                        strong { "{strings.garbage_collect_hint()}" }
                    }
                    
                    div {
                        class: "command-box",
                        code { "{gc_command}" }
                        button {
                            class: "btn-icon small",
                            title: "{strings.copy_command()}" ,
                            onclick: copy_command,
                            "{copy_button_text(copied())}"
                        }
                    }
                    
                    p {
                        class: "hint",
                        "{strings.replace_registry_container_hint()}"
                    }
                }
            }
            
            div {
                class: "dialog-footer form-actions",
                button {
                    class: "primary",
                    onclick: move |_| on_close.call(()),
                    "{strings.close()}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_state_classes_map_to_semantic_variants() {
        assert_eq!(dialog_state_class(false, false, true), "delete-dialog-state delete-dialog-confirm-state");
        assert_eq!(dialog_state_class(false, true, true), "delete-dialog-state delete-dialog-progress-state");
        assert_eq!(dialog_state_class(true, false, true), "delete-dialog-state delete-dialog-complete-state");
        assert_eq!(dialog_state_class(false, false, false), "delete-dialog-state delete-dialog-info-state");
    }

    #[test]
    fn completed_flags_clear_deleting_state() {
        assert_eq!(normalized_dialog_flags(true, true), (true, false));
        assert_eq!(normalized_dialog_flags(false, true), (false, true));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn desktop_copy_attempt_keeps_feedback_in_idle_state() {
        assert!(!desktop_copy_feedback_after_attempt());
        assert_eq!(copy_button_text(false), "📋");
        assert_eq!(copy_button_text(true), "📋");
    }
}
