//! Manifest view component

use crate::api::RegistryClient;
use crate::models::{HistoryEntry, Manifest};
use crate::state::AppState;
use crate::utils::{format_size, sorted_history_chronologically};
use dioxus::prelude::*;

fn manifest_context_matches(
    app_state: &AppState,
    registry_id: &str,
    repo_name: &str,
    tag_name: &str,
) -> bool {
    manifest_request_is_current(
        app_state.selected_registry.read().as_deref(),
        app_state.selected_repo.read().as_deref(),
        app_state.selected_tag.read().as_deref(),
        registry_id,
        repo_name,
        tag_name,
    )
}

/// Manifest details view component
#[component]
pub fn ManifestView() -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let selected_tag = app_state.selected_tag.read().clone();
    let selected_tag_name = selected_tag.clone().unwrap_or_default();

    let mut manifest = use_signal(|| None::<Manifest>);
    let mut digest = use_signal(String::new);
    let mut raw_json = use_signal(String::new);
    let mut show_raw = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    // Fetch manifest when tag changes
    let _fetch = use_resource(move || {
        let _ = (app_state.refresh_tick)();
        let registry_id = app_state.selected_registry.read().clone();
        let repo = app_state.selected_repo.read().clone();
        let tag = app_state.selected_tag.read().clone();
        async move {
            let (empty_manifest, empty_digest, empty_raw_json, collapsed_raw, not_loading) =
                reset_detail_state();

            show_raw.set(collapsed_raw);

            if let (Some(id), Some(repo_name), Some(tag_name)) = (registry_id, repo, tag) {
                if let Some(registry) = app_state.get_registry(&id) {
                    loading.set(true);
                    error.set(None);
                    manifest.set(empty_manifest.clone());
                    digest.set(empty_digest.clone());
                    raw_json.set(empty_raw_json.clone());

                    match RegistryClient::new(registry.url.clone(), registry.auth.clone()) {
                        Ok(client) => match client.get_manifest(&repo_name, &tag_name).await {
                            Ok((m, d)) => {
                                if manifest_context_matches(&app_state, &id, &repo_name, &tag_name)
                                {
                                    if let Ok(json) = serde_json::to_string_pretty(&m) {
                                        raw_json.set(json);
                                    }
                                    digest.set(d);
                                    manifest.set(Some(m));
                                    error.set(None);
                                }
                            }
                            Err(e) => {
                                if manifest_context_matches(&app_state, &id, &repo_name, &tag_name)
                                {
                                    error.set(Some(
                                        strings.failed_to_fetch_manifest(&e.to_string()),
                                    ));
                                    manifest.set(empty_manifest.clone());
                                    digest.set(empty_digest.clone());
                                    raw_json.set(empty_raw_json.clone());
                                }
                            }
                        },
                        Err(e) => {
                            if manifest_context_matches(&app_state, &id, &repo_name, &tag_name) {
                                error.set(Some(strings.failed_to_create_client(&e.to_string())));
                                manifest.set(empty_manifest.clone());
                                digest.set(empty_digest.clone());
                                raw_json.set(empty_raw_json.clone());
                            }
                        }
                    }

                    if manifest_context_matches(&app_state, &id, &repo_name, &tag_name) {
                        loading.set(not_loading);
                    }
                } else {
                    manifest.set(empty_manifest);
                    digest.set(empty_digest);
                    raw_json.set(empty_raw_json);
                    error.set(None);
                    loading.set(not_loading);
                }
            } else {
                manifest.set(empty_manifest);
                digest.set(empty_digest);
                raw_json.set(empty_raw_json);
                error.set(None);
                loading.set(not_loading);
            }
        }
    });

    let (empty_title, empty_body) = empty_state_guidance(strings);
    let panel_summary = if selected_tag.is_none() {
        format!("{empty_title}. {empty_body}")
    } else if loading() {
        strings.loading_manifest_summary(&selected_tag_name)
    } else if error().is_some() {
        strings.failed_manifest_summary(&selected_tag_name)
    } else if let Some(m) = manifest() {
        strings.detail_summary(&selected_tag_name, m.layers().len())
    } else {
        strings.pending_manifest_summary(&selected_tag_name)
    };

    rsx! {
        div {
            class: "manifest-view detail-panel",

            div {
                class: "detail-panel-header",
                div {
                    class: "detail-panel-title-group",
                    h3 { "{strings.manifest_details()}" }
                    p { class: "detail-panel-summary", "{panel_summary}" }
                }
            }

            div {
                class: "detail-panel-body",
            if selected_tag.is_none() {
                section {
                    class: "detail-empty-state",
                    h4 { "{empty_title}" }
                    p { "{empty_body}" }
                    p { "{strings.select_tag_body()}" }
                }
            } else if loading() {
                section {
                    class: "detail-section",
                    p { class: "loading", "{strings.loading_manifest()}" }
                }
            } else if let Some(err) = error() {
                section {
                    class: "detail-section",
                    p { class: "error", "{err}" }
                }
            } else if let Some(m) = manifest() {
                div {
                    class: "manifest-content",

                    // Basic info
                    section {
                        class: "manifest-section detail-section",
                        h4 { "{strings.overview()}" }
                        dl {
                            dt { "{strings.tag()}" }
                            dd { "{selected_tag.clone().unwrap_or_default()}" }
                            dt { "{strings.digest()}" }
                            dd {
                                class: "digest-value",
                                "{digest}"
                            }
                            dt { "{strings.media_type()}" }
                            dd { "{m.media_type()}" }
                            dt { "{strings.total_size()}" }
                            dd { "{format_size(m.total_size())}" }
                        }
                    }

                    // Layers
                    section {
                        class: "manifest-section detail-section",
                        h4 { "{strings.layers(m.layers().len())}" }
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
                    section {
                        class: "manifest-section detail-section",
                        h4 { "{strings.raw_json()}" }
                        p { "{strings.raw_json_help()}" }
                        button {
                            class: "secondary",
                            onclick: move |_| show_raw.set(true),
                            "{strings.show_raw_json()}"
                        }

                        if show_raw() {
                            div {
                                class: "modal-overlay raw-json-overlay",
                                onclick: move |_| show_raw.set(false),

                                div {
                                    class: raw_json_modal_class(),
                                    onclick: move |e| e.stop_propagation(),

                                    div {
                                        class: "dialog-header",
                                        h3 { "{strings.raw_json()}" }
                                    }

                                    div {
                                        class: "dialog-body raw-json-modal-body",
                                        pre {
                                            class: "raw-json",
                                            "{raw_json}"
                                        }
                                    }

                                    div {
                                        class: "dialog-footer",
                                        button {
                                            class: "secondary",
                                            onclick: move |_| show_raw.set(false),
                                            "{strings.close()}"
                                        }
                                    }
                                }
                            }
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

fn empty_state_guidance(
    strings: &'static dyn crate::i18n::Strings,
) -> (&'static str, &'static str) {
    (
        strings.select_tag_to_inspect(),
        strings.select_tag_guidance(),
    )
}

fn reset_detail_state() -> (Option<Manifest>, String, String, bool, bool) {
    (None, String::new(), String::new(), false, false)
}

fn raw_json_modal_class() -> &'static str {
    "modal raw-json-modal"
}

fn manifest_request_is_current(
    current_registry: Option<&str>,
    current_repo: Option<&str>,
    current_tag: Option<&str>,
    expected_registry: &str,
    expected_repo: &str,
    expected_tag: &str,
) -> bool {
    current_registry == Some(expected_registry)
        && current_repo == Some(expected_repo)
        && current_tag == Some(expected_tag)
}

/// History view component
#[component]
pub fn HistoryView(history: Vec<HistoryEntry>) -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let sorted = sorted_history_chronologically(&history);

    rsx! {
        div {
            class: "history-view",
            h4 { "{strings.build_history()}" }

            if sorted.is_empty() {
                p { class: "empty-message", "{strings.no_history_available()}" }
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
                                span { class: "empty-layer-badge", "{strings.empty_layer()}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Descriptor, ManifestV2};

    #[test]
    fn detail_summary_mentions_tag_and_layer_count() {
        let manifest = Manifest::V2(ManifestV2 {
            schema_version: 2,
            media_type: "application/vnd.docker.distribution.manifest.v2+json".to_string(),
            config: Descriptor {
                media_type: "application/vnd.docker.container.image.v1+json".to_string(),
                size: 7023,
                digest: "sha256:config".to_string(),
            },
            layers: vec![
                Descriptor {
                    media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".to_string(),
                    size: 128,
                    digest: "sha256:layer1".to_string(),
                },
                Descriptor {
                    media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".to_string(),
                    size: 256,
                    digest: "sha256:layer2".to_string(),
                },
            ],
        });

        let summary = crate::i18n::strings_for_locale(crate::models::Locale::En)
            .detail_summary("stable", manifest.layers().len());

        assert!(summary.contains("stable"));
        assert!(summary.contains("2 layers"));
    }

    #[test]
    fn empty_state_guidance_prompts_selection_and_preview_scope() {
        let (title, body) =
            empty_state_guidance(crate::i18n::strings_for_locale(crate::models::Locale::En));

        assert_eq!(title, "Select a tag to inspect");
        assert!(body.contains("Choose a tag"));
        assert!(body.contains("manifest overview"));
        assert!(body.contains("raw JSON"));
    }

    #[test]
    fn reset_detail_state_clears_payload_and_collapses_raw_panel() {
        let (manifest, digest, raw_json, show_raw, loading) = reset_detail_state();

        assert!(manifest.is_none());
        assert!(digest.is_empty());
        assert!(raw_json.is_empty());
        assert!(!show_raw);
        assert!(!loading);
    }

    #[test]
    fn manifest_request_is_current_requires_full_selection_match() {
        assert!(manifest_request_is_current(
            Some("registry-a"),
            Some("repo-a"),
            Some("tag-a"),
            "registry-a",
            "repo-a",
            "tag-a",
        ));

        assert!(!manifest_request_is_current(
            Some("registry-a"),
            Some("repo-a"),
            Some("tag-b"),
            "registry-a",
            "repo-a",
            "tag-a",
        ));
    }

    #[test]
    fn raw_json_modal_uses_dedicated_dialog_class() {
        assert_eq!(raw_json_modal_class(), "modal raw-json-modal");
    }
}
