//! Error display component

use crate::models::{AppError, ErrorDetails};
use crate::state::AppState;
use dioxus::prelude::*;

/// Error display component
#[component]
pub fn ErrorDisplay(error: AppError) -> Element {
    let mut show_details = use_signal(|| false);

    rsx! {
        div {
            class: "error-display",

            div {
                class: "error-header",
                span { class: "error-icon", "⚠️" }
                span { class: "error-message", "{error.message}" }

                if error.details.is_some() {
                    button {
                        class: "btn-icon small",
                        onclick: move |_| show_details.set(!show_details()),
                        if show_details() { "▼" } else { "▶" }
                    }
                }
            }

            if show_details() {
                if let Some(details) = &error.details {
                    ErrorDetailsView { details: details.clone() }
                }
            }
        }
    }
}

/// Error details view
#[component]
fn ErrorDetailsView(details: ErrorDetails) -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let mut show_headers = use_signal(|| false);
    let mut show_body = use_signal(|| false);

    rsx! {
        div {
            class: "error-details",

            if let Some(status) = details.status_code {
                div {
                    class: "detail-row",
                    span { class: "detail-label", "{strings.status_code()}" }
                    span { class: "detail-value", "{status}" }
                }
            }

            if let Some(headers) = &details.response_headers {
                div {
                    class: "detail-section",
                    button {
                        class: "secondary small",
                        onclick: move |_| show_headers.set(!show_headers()),
                        "{strings.response_headers()}"
                    }

                    if show_headers() {
                        pre {
                            class: "detail-content",
                            for (key, value) in headers {
                                "{key}: {value}\n"
                            }
                        }
                    }
                }
            }

            if let Some(body) = &details.response_body {
                div {
                    class: "detail-section",
                    button {
                        class: "secondary small",
                        onclick: move |_| show_body.set(!show_body()),
                        "{strings.response_body()}"
                    }

                    if show_body() {
                        pre {
                            class: "detail-content",
                            "{body}"
                        }
                    }
                }
            }

            if let Some(curl) = &details.curl_command {
                div {
                    class: "detail-section",
                    span { class: "detail-label", "{strings.curl_command()}" }
                    div {
                        class: "curl-command",
                        pre { "{curl}" }
                        button {
                            class: "btn-icon small",
                            title: "{strings.copy()}" ,
                            onclick: move |_| {
                                // Copy functionality would be implemented here
                            },
                            "📋"
                        }
                    }
                }
            }
        }
    }
}
