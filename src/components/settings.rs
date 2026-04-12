//! Settings panel component

use crate::models::{Locale, Theme};
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

/// Settings panel component
#[component]
pub fn Settings() -> Element {
    let mut app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let theme = app_state.theme;
    let locale = app_state.locale;
    let cache_config = app_state.cache_config;

    let mut import_text = use_signal(String::new);
    let mut export_text = use_signal(String::new);
    let mut import_error = use_signal(|| None::<String>);

    rsx! {
        div {
            class: "settings-page",

            div {
                class: "settings-page-header",

                h2 { "{strings.settings_title()}" }
                p { "{strings.settings_subtitle()}" }
            }

            div {
                class: "settings-page-body settings-panel",

                // Theme settings
                section {
                    class: "settings-section",
                    h3 { "{strings.appearance()}" }

                    div {
                        class: "form-group",
                        label { "{strings.theme()}" }
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
                            option { value: "system", "{strings.follow_system()}" }
                            option { value: "light", "{strings.light()}" }
                            option { value: "dark", "{strings.dark()}" }
                        }
                    }

                    div {
                        class: "form-group",
                        label { "{strings.language()}" }
                        select {
                            value: match locale() {
                                Locale::System => "system",
                                Locale::En => "en",
                                Locale::ZhHans => "zh-Hans",
                                Locale::ZhHant => "zh-Hant",
                            },
                            onchange: move |e| {
                                let new_locale = match e.value().as_str() {
                                    "en" => Locale::En,
                                    "zh-Hans" => Locale::ZhHans,
                                    "zh-Hant" => Locale::ZhHant,
                                    _ => Locale::System,
                                };
                                app_state.set_locale(new_locale);
                            },
                            option { value: "system", "{strings.follow_system()}" }
                            option { value: "en", "{strings.english()}" }
                            option { value: "zh-Hans", "{strings.chinese_simplified()}" }
                            option { value: "zh-Hant", "{strings.chinese_traditional()}" }
                        }
                    }
                }

                // Cache settings
                section {
                    class: "settings-section",
                    h3 { "{strings.cache()}" }

                    div {
                        class: "form-group",
                        label { "{strings.auto_refresh_interval()}" }
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
                        label { "{strings.cache_max_age()}" }
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
                        "{strings.reset_cache_settings()}"
                    }
                }

                // Import/Export
                section {
                    class: "settings-section",
                    h3 { "{strings.import_export()}" }

                    div {
                        class: "form-group",
                        label { "{strings.export_registries()}" }
                        button {
                            class: "secondary",
                            onclick: move |_| {
                                let registries = app_state.registries.read();
                                let json = crate::utils::export_registries(&registries);
                                export_text.set(json);
                            },
                            "{strings.generate_export()}"
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
                        label { "{strings.import_registries()}" }
                        textarea {
                            placeholder: "{strings.paste_exported_json_here()}",
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
                            "{strings.import_action()}"
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
