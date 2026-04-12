//! Registry configuration form component

use crate::state::AppState;
use dioxus::prelude::*;

/// Registry configuration form
#[component]
pub fn RegistryForm() -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();

    rsx! {
        div {
            class: "registry-form",
            h3 { "{strings.add_registry()}" }
            // Placeholder - will be implemented in UI tasks
        }
    }
}
