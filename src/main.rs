//! Docker Registry Manager - Main entry point

use base64::Engine;
use dioxus::prelude::*;
use docker_registry_manager::components::App;
use docker_registry_manager::state::{theme_class, AppState};

const MAIN_STYLESHEET: &str = include_str!("../assets/main.css");

#[cfg(test)]
fn root_theme_class(theme: docker_registry_manager::models::Theme) -> &'static str {
    theme_class(&theme)
}

/// Embedded app icons (base64 encoded for data URI)
const FAVICON_ICO: &[u8] = include_bytes!("../assets/app.ico");
const APP_ICON_PNG: &[u8] = include_bytes!("../assets/app.png");

/// Get WebView2 data directory in AppData
#[cfg(all(windows, not(target_arch = "wasm32")))]
fn get_webview_data_dir() -> Option<std::path::PathBuf> {
    directories::ProjectDirs::from("", "", "docker-registry-manager")
        .map(|dirs| dirs.data_dir().join("WebView2"))
}

fn main() {
    #[cfg(not(debug_assertions))]
    {
        // Release mode: hide menubar, configure WebView2 data directory
        let mut config = dioxus::desktop::Config::new().with_menu(None).with_window(
            dioxus::desktop::WindowBuilder::new().with_title("Docker Registry Manager"),
        );

        // Set WebView2 data directory on Windows
        #[cfg(windows)]
        if let Some(data_dir) = get_webview_data_dir() {
            config = config.with_data_directory(data_dir);
        }

        dioxus::LaunchBuilder::desktop()
            .with_cfg(config)
            .launch(Root);
    }

    #[cfg(debug_assertions)]
    {
        // Debug mode: also configure WebView2 data directory
        #[cfg(windows)]
        {
            let mut config = dioxus::desktop::Config::new();
            if let Some(data_dir) = get_webview_data_dir() {
                config = config.with_data_directory(data_dir);
            }
            dioxus::LaunchBuilder::desktop()
                .with_cfg(config)
                .launch(Root);
        }

        #[cfg(not(windows))]
        dioxus::launch(Root);
    }
}

#[cfg(test)]
fn main_stylesheet() -> &'static str {
    MAIN_STYLESHEET
}

#[cfg(test)]
fn icon_data_uris() -> (String, String) {
    let favicon_ico = base64::engine::general_purpose::STANDARD.encode(FAVICON_ICO);
    let app_icon_png = base64::engine::general_purpose::STANDARD.encode(APP_ICON_PNG);
    (
        format!("data:image/x-icon;base64,{}", favicon_ico),
        format!("data:image/png;base64,{}", app_icon_png),
    )
}

#[component]
fn Root() -> Element {
    let mut app_state = use_context_provider(AppState::load_from_storage);

    // Create base64 data URIs for embedded icons.
    let favicon_base64 = base64::engine::general_purpose::STANDARD.encode(FAVICON_ICO);
    let favicon_uri = format!("data:image/x-icon;base64,{}", favicon_base64);
    let app_icon_png_base64 = base64::engine::general_purpose::STANDARD.encode(APP_ICON_PNG);
    let app_icon_png_uri = format!("data:image/png;base64,{}", app_icon_png_base64);

    let mut is_ready = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            true
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    });

    let current_theme_class = theme_class(&(app_state.theme)());

    rsx! {
        document::Style {
            {MAIN_STYLESHEET}
        }
        document::Link { rel: "icon", href: "{favicon_uri}" }
        document::Link { rel: "icon", r#type: "image/png", href: "{app_icon_png_uri}" }
        document::Link { rel: "apple-touch-icon", href: "{app_icon_png_uri}" }

        div {
            class: "app-container",
            "data-theme": current_theme_class,

            if is_ready() {
                App {}
            } else {
                docker_registry_manager::components::EncryptionPrompt {
                    on_ready: move |_| {
                        app_state.reload_from_storage();
                        is_ready.set(true);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{icon_data_uris, main_stylesheet, root_theme_class};
    use docker_registry_manager::models::Theme;

    #[test]
    fn main_stylesheet_embeds_console_workspace_styles() {
        let stylesheet = main_stylesheet();
        assert!(stylesheet.contains("Docker Registry Manager - Console workspace styles"));
        assert!(stylesheet.contains(".app-shell"));
    }

    #[test]
    fn root_theme_class_tracks_explicit_theme_selection() {
        assert_eq!(root_theme_class(Theme::Light), "light");
        assert_eq!(root_theme_class(Theme::Dark), "dark");
        assert_eq!(root_theme_class(Theme::System), "system");
    }

    #[test]
    fn icons_are_embedded_as_data_uris() {
        let (favicon_ico, app_icon_png) = icon_data_uris();
        assert!(favicon_ico.starts_with("data:image/x-icon;base64,"));
        assert!(app_icon_png.starts_with("data:image/png;base64,"));
    }
}
