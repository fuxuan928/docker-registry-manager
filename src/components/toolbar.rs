//! Toolbar component

use crate::state::AppState;
use dioxus::prelude::*;

fn toolbar_status_copy(
    strings: &'static dyn crate::i18n::Strings,
    context_path: &str,
) -> (&'static str, String) {
    let detail = if context_path == strings.no_selection() {
        strings.context_label(strings.no_selection())
    } else {
        strings.context_label(context_path)
    };

    (strings.ready(), detail)
}

fn display_registry_name(
    selected_registry_id: Option<&str>,
    registry_name: Option<&str>,
) -> Option<String> {
    match (selected_registry_id, registry_name) {
        (_, Some(name)) => Some(name.to_string()),
        (Some(id), None) => Some(id.to_string()),
        (None, None) => None,
    }
}

/// Application toolbar component
#[component]
pub fn Toolbar(show_settings: Signal<bool>) -> Element {
    let mut app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let selected_registry = app_state.selected_registry.read().clone();
    let selected_repo = app_state.selected_repo.read().clone();
    let selected_tag = app_state.selected_tag.read().clone();
    let selected_registry_name = selected_registry
        .as_deref()
        .and_then(|id| app_state.get_registry(id).map(|registry| registry.name));
    let registry_label = display_registry_name(
        selected_registry.as_deref(),
        selected_registry_name.as_deref(),
    );

    let context_path = match (&registry_label, &selected_repo, &selected_tag) {
        (Some(registry), Some(repo), Some(tag)) => format!("{registry} / {repo} / {tag}"),
        (Some(registry), Some(repo), None) => format!("{registry} / {repo}"),
        (Some(registry), None, None) => registry.clone(),
        _ => strings.no_selection().to_string(),
    };

    let context_summary = match (&selected_registry, &selected_repo, &selected_tag) {
        (_, _, Some(_)) => strings.selected_tag_context().to_string(),
        (_, Some(_), None) => strings.selected_repository_context().to_string(),
        (Some(_), None, None) => strings.selected_registry_context().to_string(),
        _ => strings.context_guidance().to_string(),
    };

    let (status_label, status_detail) = toolbar_status_copy(strings, &context_path);

    rsx! {
        header {
            class: "topbar",

            div {
                class: "topbar-title-group",

                h1 { "{strings.app_title()}" }

                div {
                    class: "topbar-context",

                    div {
                        class: "context-path",
                        "{context_path}"
                    }

                    div {
                        class: "context-summary",
                        "{context_summary}"
                    }
                }
            }

            div {
                class: "topbar-actions",

                div {
                    class: "toolbar-status-slot",

                    span {
                        class: "toolbar-status-label",
                        "{status_label}"
                    }

                    span {
                        class: "toolbar-status-detail",
                        "{status_detail}"
                    }
                }

                button {
                    class: "btn-icon",
                    title: "{strings.refresh()}",
                    disabled: selected_registry.is_none(),
                    onclick: move |_| app_state.request_refresh(),
                    "{strings.refresh()}"
                }

                button {
                    class: if show_settings() { "btn-icon active" } else { "btn-icon" },
                    title: "{strings.settings()}",
                    onclick: move |_| show_settings.set(!show_settings()),
                    if show_settings() {
                        "{strings.close_settings()}"
                    } else {
                        "{strings.open_settings()}"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbar_status_copy_reports_ready_state_without_selection() {
        let strings = crate::i18n::strings_for_locale(crate::models::Locale::En);
        let (label, detail) = toolbar_status_copy(strings, "No selection");

        assert_eq!(label, "Ready");
        assert!(detail.contains("No selection"));
    }

    #[test]
    fn toolbar_status_copy_includes_context_when_available() {
        let strings = crate::i18n::strings_for_locale(crate::models::Locale::En);
        let (_, detail) = toolbar_status_copy(strings, "registry-a / repo-a");

        assert!(detail.contains("registry-a / repo-a"));
    }

    #[test]
    fn display_registry_name_prefers_registry_name_over_id() {
        let label = display_registry_name(Some("registry-id"), Some("Production Registry"));

        assert_eq!(label.as_deref(), Some("Production Registry"));
    }
}
