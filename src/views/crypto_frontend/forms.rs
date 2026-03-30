use crate::{model::crypto::EncryptedPayload, views::crypto_style::*};
use dioxus::prelude::*;
use crate::crypto::SolanaEncryption;
use super::{components::*, validation::*};

#[component]
pub fn EncryptionForm() -> Element {
    let mut title = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut recipient_public_key = use_signal(String::new);
    let mut my_private_key = use_signal(String::new);
    let mut my_public_key = use_signal(String::new);
    let mut encrypted_result = use_signal(|| Option::<String>::None);
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut is_processing = use_signal(|| false);

    let encrypt_message = move |_| {
        spawn(async move {
            is_processing.set(true);
            encrypted_result.set(None);
            error_message.set(None);

            let title_val = title.read().clone();
            let message_val = message.read().clone();
            let recipient_key = recipient_public_key.read().clone();
            let private_key = my_private_key.read().clone();
            let public_key = my_public_key.read().clone();

            // Validate inputs
            if let Err(validation_error) = validate_encryption_inputs(
                &title_val,
                &message_val,
                &recipient_key,
                &private_key,
                &public_key,
            ) {
                error_message.set(Some(validation_error));
                is_processing.set(false);
                return;
            }

            // Perform encryption
            match SolanaEncryption::encrypt_for_recipient(
                &message_val,
                &recipient_key,
                &private_key,
                &public_key,
            ) {
                Ok(encrypted_payload) => {
                    let result = format!(
                        "Title: {}\nEncrypted Payload:\n{}",
                        title_val,
                        serde_json::to_string_pretty(&encrypted_payload).unwrap_or_default()
                    );
                    encrypted_result.set(Some(result));
                }
                Err(e) => {
                    error_message.set(Some(e.to_string()));
                }
            }

            is_processing.set(false);
        });
    };

    rsx! {
        div { class: CARD,
            CardHeader { title: "Encrypt Message" }

            div { class: FORM_CONTAINER,
                InputField {
                    label: "Title",
                    placeholder: "Message title",
                    input_type: "text",
                    value: title.read().clone(),
                    on_input: move |value| title.set(value)
                }

                TextAreaField {
                    label: "Message",
                    placeholder: "Your message to encrypt",
                    rows: 4,
                    value: message.read().clone(),
                    on_input: move |value| message.set(value)
                }

                InputField {
                    label: "Recipient Public Key (Base58)",
                    placeholder: "Recipient's Solana public key",
                    input_type: "text",
                    value: recipient_public_key.read().clone(),
                    on_input: move |value| recipient_public_key.set(value)
                }

                InputField {
                    label: "Your Private Key (Base58)",
                    placeholder: "Your Solana private key (not stored)",
                    input_type: "password",
                    value: my_private_key.read().clone(),
                    on_input: move |value| my_private_key.set(value)
                }

                InputField {
                    label: "Your Public Key (Base58)",
                    placeholder: "Your Solana public key",
                    input_type: "text",
                    value: my_public_key.read().clone(),
                    on_input: move |value| my_public_key.set(value)
                }

                ActionButton {
                    text: if *is_processing.read() { "Encrypting..." } else { "Encrypt Message" },
                    button_type: ButtonType::Primary,
                    disabled: *is_processing.read(),
                    on_click: encrypt_message
                }

                if let Some(error) = error_message.read().as_ref() {
                    ErrorMessage { message: error.clone() }
                }

                if let Some(result) = encrypted_result.read().as_ref() {
                    SuccessMessage {
                        title: "Encryption Result:",
                        content: result.clone(),
                        is_code: true
                    }
                }
            }
        }
    }
}

#[component]
pub fn DecryptionForm() -> Element {
    let mut encrypted_payload_input = use_signal(String::new);
    let mut my_private_key = use_signal(String::new);
    let mut decrypted_result = use_signal(|| Option::<String>::None);
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut is_processing = use_signal(|| false);

    let decrypt_message = move |_| {
        spawn(async move {
            is_processing.set(true);
            decrypted_result.set(None);
            error_message.set(None);

            let payload_str = encrypted_payload_input.read().clone();
            let private_key = my_private_key.read().clone();

            // Validate inputs
            if let Err(validation_error) = validate_decryption_inputs(&payload_str, &private_key) {
                error_message.set(Some(validation_error));
                is_processing.set(false);
                return;
            }

            // Parse and decrypt
            match serde_json::from_str::<EncryptedPayload>(&payload_str) {
                Ok(payload) => {
                    match SolanaEncryption::decrypt_from_sender(&payload, &private_key) {
                        Ok(decrypted_msg) => {
                            decrypted_result.set(Some(decrypted_msg));
                        }
                        Err(e) => {
                            error_message.set(Some(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    error_message.set(Some(format!("Invalid encrypted payload format: {e}")));
                }
            }

            is_processing.set(false);
        });
    };

    rsx! {
        div { class: CARD,
            CardHeader { title: "Decrypt Message" }

            div { class: FORM_CONTAINER,
                TextAreaField {
                    label: "Encrypted Payload (JSON)",
                    placeholder: "Paste the encrypted JSON payload here",
                    rows: 8,
                    value: encrypted_payload_input.read().clone(),
                    on_input: move |value| encrypted_payload_input.set(value)
                }

                InputField {
                    label: "Your Private Key (Base58)",
                    placeholder: "Your Solana private key",
                    input_type: "password",
                    value: my_private_key.read().clone(),
                    on_input: move |value| my_private_key.set(value)
                }

                ActionButton {
                    text: if *is_processing.read() { "Decrypting..." } else { "Decrypt Message" },
                    button_type: ButtonType::Success,
                    disabled: *is_processing.read(),
                    on_click: decrypt_message
                }

                if let Some(error) = error_message.read().as_ref() {
                    ErrorMessage { message: error.clone() }
                }

                if let Some(result) = decrypted_result.read().as_ref() {
                    SuccessMessage {
                        title: "Decrypted Message:",
                        content: result.clone(),
                        is_code: false
                    }
                }
            }
        }
    }
}
