//! Docker Registry Manager - Main entry point

use dioxus::prelude::*;
use docker_registry_manager::components::App;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

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
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        App {}
    }
}
