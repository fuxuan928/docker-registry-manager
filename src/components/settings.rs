//! Settings panel component

use crate::models::Theme;
use crate::state::AppState;
use dioxus::prelude::*;

fn cache_config_with_refresh_interval(
    mut config: crate::models::CacheConfig,
    refresh_interval: u64,
) -> crate::models::CacheConfig {
    config.refresh_interval = refresh_interval;
    config
}

fn cache_config_with_max_age(
    mut config: crate::models::CacheConfig,
    max_age: u64,
) -> crate::models::CacheConfig {
    config.max_age = max_age;
    config
}

fn settings_page_copy() -> (&'static str, &'static str) {
    (
        "Settings & Preferences",
        "Configure the workspace, console behavior, and registry manager defaults.",
    )
}

/// Settings panel component
#[component]
pub fn Settings() -> Element {
    let mut app_state = use_context::<AppState>();
    let theme = app_state.theme;
    let cache_config = app_state.cache_config;
    let (page_title, page_subtitle) = settings_page_copy();

    let mut import_text = use_signal(String::new);
    let mut export_text = use_signal(String::new);
    let mut import_error = use_signal(|| None::<String>);

    rsx! {
        div {
            class: "settings-page",

            div {
                class: "settings-page-header",

                h2 { "{page_title}" }
                p { "{page_subtitle}" }
            }

            div {
                class: "settings-page-body settings-panel",

                // Theme settings
                section {
                    class: "settings-section",
                    h3 { "Appearance" }

                    div {
                        class: "form-group",
                        label { "Theme" }
                        select {
                            value: match theme() {
                                Theme::Light => "light",
                                Theme::Dark => "dark",
                                Theme::System => "system",
                            },
                            onchange: move |e| {
                                let new_theme = match e.value().as_str() {
                                    "light" => Theme::Light,
                                    "dark" => Theme::Dark,
                                    _ => Theme::System,
                                };
                                app_state.set_theme(new_theme);
                            },
                            option { value: "system", "Follow System" }
                            option { value: "light", "Light" }
                            option { value: "dark", "Dark" }
                        }
                    }
                }

                // Cache settings
                section {
                    class: "settings-section",
                    h3 { "Cache" }

                    div {
                        class: "form-group",
                        label { "Auto-refresh interval (seconds, 0 = disabled)" }
                        input {
                            r#type: "number",
                            min: "0",
                            value: "{cache_config().refresh_interval}",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse() {
                                    app_state.set_cache_config(cache_config_with_refresh_interval(cache_config(), val));
                                }
                            },
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Cache max age (seconds)" }
                        input {
                            r#type: "number",
                            min: "60",
                            value: "{cache_config().max_age}",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse() {
                                    app_state.set_cache_config(cache_config_with_max_age(cache_config(), val));
                                }
                            },
                        }
                    }

                    button {
                        class: "danger",
                        onclick: move |_| {
                            app_state.set_cache_config(crate::models::CacheConfig::default());
                        },
                        "Reset Cache Settings"
                    }
                }

                // Import/Export
                section {
                    class: "settings-section",
                    h3 { "Import / Export" }

                    div {
                        class: "form-group",
                        label { "Export Registries" }
                        button {
                            class: "secondary",
                            onclick: move |_| {
                                let registries = app_state.registries.read();
                                let json = crate::utils::export_registries(&registries);
                                export_text.set(json);
                            },
                            "Generate Export"
                        }

                        if !export_text().is_empty() {
                            textarea {
                                readonly: true,
                                value: "{export_text}",
                                rows: "10",
                            }
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Import Registries (paste JSON)" }
                        textarea {
                            placeholder: "Paste exported JSON here...",
                            value: "{import_text}",
                            oninput: move |e| {
                                import_text.set(e.value());
                                import_error.set(None);
                            },
                            rows: "5",
                        }

                        if let Some(err) = import_error() {
                            p { class: "error", "{err}" }
                        }

                        button {
                            class: "primary",
                            disabled: import_text().is_empty(),
                            onclick: move |_| {
                                match crate::utils::import_registries(&import_text()) {
                                    Ok(configs) => {
                                        for config in configs {
                                            app_state.add_registry(config);
                                        }
                                        import_text.set(String::new());
                                        import_error.set(None);
                                    }
                                    Err(e) => {
                                        import_error.set(Some(e));
                                    }
                                }
                            },
                            "Import"
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

    #[test]
    fn settings_page_copy_describes_console_page_shell() {
        let (title, subtitle) = settings_page_copy();

        assert_eq!(title, "Settings & Preferences");
        assert!(subtitle.contains("workspace"));
        assert!(subtitle.contains("registry manager"));
    }

    #[test]
    fn cache_config_helpers_update_only_requested_field() {
        let base = crate::models::CacheConfig {
            refresh_interval: 30,
            max_age: 300,
        };

        let updated_refresh = cache_config_with_refresh_interval(base.clone(), 45);
        let updated_max_age = cache_config_with_max_age(base, 600);

        assert_eq!(updated_refresh.refresh_interval, 45);
        assert_eq!(updated_refresh.max_age, 300);
        assert_eq!(updated_max_age.refresh_interval, 30);
        assert_eq!(updated_max_age.max_age, 600);
    }
}
