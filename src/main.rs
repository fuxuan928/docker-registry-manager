//! Docker Registry Manager - Main entry point

use base64::Engine;
use dioxus::prelude::*;
use docker_registry_manager::components::App;

/// Embedded CSS styles
const MAIN_CSS: &str = include_str!("../assets/main.css");

/// Embedded favicon (base64 encoded for data URI)
const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");

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
        let mut config = dioxus::desktop::Config::new()
            .with_menu(None)
            .with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("Docker Registry Manager")
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

#[component]
fn Root() -> Element {
    // Create base64 data URI for favicon
    let favicon_base64 = base64::engine::general_purpose::STANDARD.encode(FAVICON_ICO);
    let favicon_uri = format!("data:image/x-icon;base64,{}", favicon_base64);
    
    rsx! {
        // Inject CSS as inline style
        style { {MAIN_CSS} }
        // Inject favicon as data URI
        document::Link { rel: "icon", href: "{favicon_uri}" }
        App {}
    }
}
