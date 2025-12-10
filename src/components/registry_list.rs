//! Registry list component

use dioxus::prelude::*;
use crate::state::AppState;
use crate::models::{AuthConfig, ConnectionStatus, RegistryConfig};

/// Registry list sidebar component
#[component]
pub fn RegistryList() -> Element {
    let mut app_state = use_context::<AppState>();
    let mut show_form = use_signal(|| false);
    let mut editing_id = use_signal(|| None::<String>);
    let mut delete_confirm_id = use_signal(|| None::<String>);
    
    let registries = app_state.registries.read().clone();
    let selected = app_state.selected_registry.read().clone();
    
    // Get registry name for delete confirmation
    let delete_registry_name = delete_confirm_id()
        .as_ref()
        .and_then(|id| registries.iter().find(|r| &r.id == id))
        .map(|r| r.name.clone())
        .unwrap_or_default();
    
    rsx! {
        div {
            class: "registry-list",
            
            div {
                class: "registry-header",
                h3 { "Registries" }
                button {
                    class: "btn-icon",
                    onclick: move |_| {
                        editing_id.set(None);
                        show_form.set(true);
                    },
                    "+"
                }
            }
            
            if registries.is_empty() {
                p {
                    class: "empty-message",
                    "No registries configured"
                }
            }
            
            for registry in registries.iter() {
                RegistryItem {
                    registry: registry.clone(),
                    is_selected: selected.as_ref() == Some(&registry.id),
                    on_select: move |id: String| {
                        app_state.select_registry(Some(id));
                    },
                    on_edit: move |id: String| {
                        editing_id.set(Some(id));
                        show_form.set(true);
                    },
                    on_delete: move |id: String| {
                        delete_confirm_id.set(Some(id));
                    },
                }
            }
            
            if show_form() {
                RegistryFormModal {
                    editing_id: editing_id(),
                    on_close: move |_| show_form.set(false),
                    on_save: move |config: RegistryConfig| {
                        if editing_id().is_some() {
                            let id = config.id.clone();
                            app_state.update_registry(&id, config);
                        } else {
                            app_state.add_registry(config);
                        }
                        show_form.set(false);
                    },
                }
            }
            
            // Delete confirmation dialog
            if delete_confirm_id().is_some() {
                div {
                    class: "modal-overlay",
                    onclick: move |_| delete_confirm_id.set(None),
                    
                    div {
                        class: "modal delete-dialog",
                        onclick: move |e| e.stop_propagation(),
                        
                        h3 { "Delete Registry" }
                        
                        div {
                            class: "delete-confirm",
                            p {
                                "Are you sure you want to delete registry "
                                strong { "{delete_registry_name}" }
                                "?"
                            }
                            
                            p { class: "warning", "⚠️ This will remove the registry from your list. Your images on the registry will not be affected." }
                            
                            div {
                                class: "form-actions",
                                button {
                                    class: "secondary",
                                    onclick: move |_| delete_confirm_id.set(None),
                                    "Cancel"
                                }
                                button {
                                    class: "danger",
                                    onclick: move |_| {
                                        if let Some(id) = delete_confirm_id() {
                                            app_state.delete_registry(&id);
                                        }
                                        delete_confirm_id.set(None);
                                    },
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Single registry item in the list
#[component]
fn RegistryItem(
    registry: RegistryConfig,
    is_selected: bool,
    on_select: EventHandler<String>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let status_class = match &registry.status {
        ConnectionStatus::Connected => "connected",
        ConnectionStatus::Disconnected => "disconnected",
        ConnectionStatus::Error(_) => "error",
        ConnectionStatus::Unknown => "unknown",
    };
    
    let id = registry.id.clone();
    let id_edit = registry.id.clone();
    let id_delete = registry.id.clone();
    
    rsx! {
        div {
            class: if is_selected { "list-item selected" } else { "list-item" },
            onclick: move |_| on_select.call(id.clone()),
            
            span {
                class: "status-indicator {status_class}",
            }
            
            div {
                class: "registry-info",
                span { class: "registry-name", "{registry.name}" }
                span { class: "registry-url", "{registry.url}" }
            }
            
            div {
                class: "registry-actions",
                button {
                    class: "btn-icon small",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_edit.call(id_edit.clone());
                    },
                    "✎"
                }
                button {
                    class: "btn-icon small danger",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_delete.call(id_delete.clone());
                    },
                    "×"
                }
            }
        }
    }
}

/// Modal form for adding/editing registries
#[component]
fn RegistryFormModal(
    editing_id: Option<String>,
    on_close: EventHandler<()>,
    on_save: EventHandler<RegistryConfig>,
) -> Element {
    let app_state = use_context::<AppState>();
    
    // Get existing config if editing
    let existing = editing_id.as_ref().and_then(|id| app_state.get_registry(id));
    
    let mut name = use_signal(|| existing.as_ref().map(|r| r.name.clone()).unwrap_or_default());
    let mut url = use_signal(|| existing.as_ref().map(|r| r.url.clone()).unwrap_or_default());
    let mut auth_type = use_signal(|| {
        existing.as_ref().map(|r| match &r.auth {
            AuthConfig::Anonymous => "anonymous",
            AuthConfig::BasicAuth { .. } => "basic",
            AuthConfig::BearerToken { .. } => "bearer",
            AuthConfig::TlsCert { .. } => "tls",
        }).unwrap_or("anonymous").to_string()
    });
    let mut username = use_signal(|| {
        existing.as_ref().and_then(|r| match &r.auth {
            AuthConfig::BasicAuth { username, .. } => Some(username.clone()),
            _ => None,
        }).unwrap_or_default()
    });
    let mut password = use_signal(String::new);
    let mut token = use_signal(String::new);
    
    let title = if editing_id.is_some() { "Edit Registry" } else { "Add Registry" };
    
    rsx! {
        div {
            class: "modal-overlay",
            // 不再点击空白关闭
            
            div {
                class: "modal",
                onclick: move |e| e.stop_propagation(),
                
                h3 { "{title}" }
                
                form {
                    onsubmit: move |e| {
                        e.prevent_default();
                        
                        let auth = match auth_type().as_str() {
                            "basic" => AuthConfig::BasicAuth {
                                username: username(),
                                password: password(),
                                encrypted_password: String::new(),
                            },
                            "bearer" => AuthConfig::BearerToken {
                                token: token(),
                                encrypted_token: String::new(),
                            },
                            _ => AuthConfig::Anonymous,
                        };
                        
                        let config = if let Some(id) = &editing_id {
                            RegistryConfig {
                                id: id.clone(),
                                name: name(),
                                url: url(),
                                auth,
                                status: ConnectionStatus::Unknown,
                            }
                        } else {
                            RegistryConfig::new(name(), url(), auth)
                        };
                        
                        on_save.call(config);
                    },
                    
                    div {
                        class: "form-group",
                        label { "Name" }
                        input {
                            r#type: "text",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                            required: true,
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { "URL" }
                        input {
                            r#type: "url",
                            value: "{url}",
                            placeholder: "https://registry.example.com",
                            oninput: move |e| url.set(e.value()),
                            required: true,
                        }
                    }
                    
                    div {
                        class: "form-group",
                        label { "Authentication" }
                        select {
                            value: "{auth_type}",
                            onchange: move |e| auth_type.set(e.value()),
                            option { value: "anonymous", "Anonymous" }
                            option { value: "basic", "Basic Auth" }
                            option { value: "bearer", "Bearer Token" }
                        }
                    }
                    
                    if auth_type() == "basic" {
                        div {
                            class: "form-group",
                            label { "Username" }
                            input {
                                r#type: "text",
                                value: "{username}",
                                oninput: move |e| username.set(e.value()),
                            }
                        }
                        div {
                            class: "form-group",
                            label { "Password" }
                            input {
                                r#type: "password",
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                            }
                        }
                    }
                    
                    if auth_type() == "bearer" {
                        div {
                            class: "form-group",
                            label { "Token" }
                            input {
                                r#type: "password",
                                value: "{token}",
                                oninput: move |e| token.set(e.value()),
                            }
                        }
                    }
                    
                    div {
                        class: "form-actions",
                        button {
                            r#type: "button",
                            class: "secondary",
                            onclick: move |_| on_close.call(()),
                            "Cancel"
                        }
                        button {
                            r#type: "submit",
                            class: "primary",
                            "Save"
                        }
                    }
                }
            }
        }
    }
}
