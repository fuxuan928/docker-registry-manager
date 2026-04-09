//! Main App component

use super::{ManifestView, RegistryList, RepositoryList, Settings, TagList, Toolbar};
use crate::state::AppState;
use dioxus::prelude::*;

/// Main application component
#[component]
pub fn App() -> Element {
    let _app_state = use_context::<AppState>();
    let show_settings = use_signal(|| false);

    rsx! {
        div {
            class: "app-shell",

            Toolbar { show_settings }

            div {
                class: "app-body",

                aside {
                    class: "left-rail",
                    RegistryList {}
                }

                main {
                    class: "workspace",

                    section {
                        class: if show_settings() { "workspace-content settings-open" } else { "workspace-content" },

                        if show_settings() {
                            Settings {}
                        } else {
                            div {
                                class: "workspace-columns",

                                section {
                                    class: "repositories-panel",
                                    RepositoryList {}
                                }

                                section {
                                    class: "tags-panel",
                                    TagList {}
                                }
                            }
                        }
                    }
                }

                if !show_settings() {
                    aside {
                        class: "detail-rail",
                        ManifestView {}
                    }
                }
            }
        }
    }
}
