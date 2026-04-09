use crate::storage::{get_storage, init_key};
use dioxus::prelude::*;
use sha2::{Digest, Sha256};

#[component]
pub fn EncryptionPrompt(on_ready: EventHandler<()>) -> Element {
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let is_first_run = !get_storage().has_config();

    let mut handle_submit = move || {
        let pwd = password.read();
        if pwd.is_empty() {
            error.set(Some("Password cannot be empty.".to_string()));
            return;
        }

        // Hash password to get a 32-byte key
        let mut hasher = Sha256::new();
        hasher.update(pwd.as_bytes());
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);

        // Initialize or replace the in-memory runtime key for this session.
        if let Err(e) = init_key(key) {
            error.set(Some(format!("Encryption error: {}", e)));
            return;
        }

        // Verify if the key is correct if not first run
        if !is_first_run {
            match get_storage().verify_encryption_verifier() {
                Ok(true) => match get_storage().load_registries() {
                    Ok(_) => on_ready.call(()),
                    Err(e) => error.set(Some(format!(
                        "Incorrect password or corrupt configuration. Error: {}",
                        e
                    ))),
                },
                Ok(false) => match get_storage().load_registries() {
                    Ok(_) => {
                        let _ = get_storage().save_encryption_verifier();
                        on_ready.call(());
                    }
                    Err(_) => {
                        error.set(Some(
                            "Incorrect password or corrupt configuration.".to_string(),
                        ));
                    }
                },
                Err(e) => {
                    error.set(Some(format!(
                        "Incorrect password or corrupt configuration. Error: {}",
                        e
                    )));
                }
            }
        } else {
            match get_storage().save_encryption_verifier() {
                Ok(_) => on_ready.call(()),
                Err(e) => error.set(Some(format!(
                    "Failed to save encryption setup. Error: {}",
                    e
                ))),
            }
        }
    };

    let mut handle_reset = move |_| {
        let _ = get_storage().clear_all();
        // Since OnceLock cannot be reset, we must inform the user to restart
        error.set(Some(
            "Configuration cleared. Please restart the application to set a new key.".to_string(),
        ));
    };

    rsx! {
        div {
            class: "encryption-prompt-overlay",
            div {
                class: "encryption-prompt-card",
                h2 { if is_first_run { "Setup Encryption" } else { "Unlock Configuration" } }
                p {
                    if is_first_run {
                        "Set a password to protect your registry credentials. This password will be required every time you start the app."
                    } else {
                        "Enter your password to decrypt your registry configurations."
                    }
                }

                div {
                    class: "form-group",
                    label { "Password" }
                    input {
                        r#type: "password",
                        value: "{password}",
                        placeholder: "Enter password...",
                        oninput: move |evt| password.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                handle_submit();
                            }
                        }
                    }
                }

                if let Some(err) = error() {
                    div { class: "error-message", "{err}" }
                }

                div {
                    class: "button-group",
                    button {
                        class: "primary-button",
                        onclick: move |_| handle_submit(),
                        if is_first_run { "Set Key" } else { "Unlock" }
                    }
                    if !is_first_run && error().is_some() {
                        button {
                            class: "danger-button",
                            onclick: move |evt| handle_reset(evt),
                            "Reset Configuration"
                        }
                    }
                }
            }
        }
    }
}
