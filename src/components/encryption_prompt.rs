use crate::state::AppState;
use crate::storage::{get_storage, init_key};
use dioxus::prelude::*;
use sha2::{Digest, Sha256};

#[component]
pub fn EncryptionPrompt(on_ready: EventHandler<()>) -> Element {
    let app_state = use_context::<AppState>();
    let strings = app_state.strings();
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let is_first_run = !get_storage().has_config();

    let mut handle_submit = move || {
        let pwd = password.read();
        if pwd.is_empty() {
            error.set(Some(strings.password_cannot_be_empty().to_string()));
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
            error.set(Some(strings.encryption_error(&e.to_string())));
            return;
        }

        // Verify if the key is correct if not first run
        if !is_first_run {
            match get_storage().verify_encryption_verifier() {
                Ok(true) => match get_storage().load_registries() {
                    Ok(_) => on_ready.call(()),
                    Err(e) => error.set(Some(
                        strings.incorrect_password_or_corrupt_config_error(&e.to_string()),
                    )),
                },
                Ok(false) => match get_storage().load_registries() {
                    Ok(_) => {
                        let _ = get_storage().save_encryption_verifier();
                        on_ready.call(());
                    }
                    Err(_) => {
                        error.set(Some(
                            strings.incorrect_password_or_corrupt_config().to_string(),
                        ));
                    }
                },
                Err(e) => {
                    error.set(Some(
                        strings.incorrect_password_or_corrupt_config_error(&e.to_string()),
                    ));
                }
            }
        } else {
            match get_storage().save_encryption_verifier() {
                Ok(_) => on_ready.call(()),
                Err(e) => error.set(Some(
                    strings.failed_to_save_encryption_setup(&e.to_string()),
                )),
            }
        }
    };

    let mut handle_reset = move |_| {
        let _ = get_storage().clear_all();
        // Since OnceLock cannot be reset, we must inform the user to restart
        error.set(Some(strings.configuration_cleared_restart().to_string()));
    };

    rsx! {
        div {
            class: "encryption-prompt-overlay",
            div {
                class: "encryption-prompt-card",
                h2 { if is_first_run { "{strings.setup_encryption()}" } else { "{strings.unlock_configuration()}" } }
                p {
                    if is_first_run {
                        "{strings.setup_encryption_help()}"
                    } else {
                        "{strings.unlock_configuration_help()}"
                    }
                }

                div {
                    class: "form-group",
                    label { "{strings.password()}" }
                    input {
                        r#type: "password",
                        value: "{password}",
                        placeholder: "{strings.enter_password()}",
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
                        if is_first_run { "{strings.set_key()}" } else { "{strings.unlock()}" }
                    }
                    if !is_first_run && error().is_some() {
                        button {
                            class: "danger-button",
                            onclick: move |evt| handle_reset(evt),
                            "{strings.reset_configuration()}"
                        }
                    }
                }
            }
        }
    }
}
