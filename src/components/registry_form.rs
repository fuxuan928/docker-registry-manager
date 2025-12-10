//! Registry configuration form component

use dioxus::prelude::*;

/// Registry configuration form
#[component]
pub fn RegistryForm() -> Element {
    rsx! {
        div {
            class: "registry-form",
            h3 { "Add Registry" }
            // Placeholder - will be implemented in UI tasks
        }
    }
}
