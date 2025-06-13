// components.rs - Frontend UI components
use dioxus::prelude::*;
//use crate::styles::*;
use crate::{crypto::{
    //CryptoError,
    EncryptedPayload, SolanaEncryption}, views::crypto_style::*};

#[component]
pub fn App() -> Element {
    rsx! {
        div { class: APP_CONTAINER,
            div { class: MAIN_WRAPPER,
                AppHeader {}
                div { class: GRID_LAYOUT,
                    EncryptionForm {}
                    DecryptionForm {}
                }
            }
        }
    }
}

#[component]
fn AppHeader() -> Element {
    rsx! {
        h1 { class: MAIN_TITLE,
            "Solana Wallet Encryption Demo"
        }
    }
}

#[component]
pub fn EncryptionForm() -> Element {
    let mut title = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut recipient_public_key = use_signal(|| String::new());
    let mut my_private_key = use_signal(|| String::new());
    let mut my_public_key = use_signal(|| String::new());
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
                &title_val, &message_val, &recipient_key, &private_key, &public_key
            ) {
                error_message.set(Some(validation_error));
                is_processing.set(false);
                return;
            }

            // Perform encryption
            match SolanaEncryption::encrypt_for_recipient(&message_val, &recipient_key, &private_key, &public_key) {
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
    let mut encrypted_payload_input = use_signal(|| String::new());
    let mut my_private_key = use_signal(|| String::new());
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
                    error_message.set(Some(format!("Invalid encrypted payload format: {}", e)));
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

// Reusable UI Components

#[derive(Props, Clone, PartialEq)]
struct CardHeaderProps {
    title: String,
}

#[component]
fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        h2 { class: CARD_TITLE, "{props.title}" }
    }
}

#[derive(Props, Clone, PartialEq)]
struct InputFieldProps {
    label: String,
    placeholder: String,
    input_type: String,
    value: String,
    on_input: EventHandler<String>,
}

#[component]
fn InputField(props: InputFieldProps) -> Element {
    rsx! {
        div {
            label { class: LABEL, "{props.label}" }
            input {
                class: INPUT,
                r#type: "{props.input_type}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                oninput: move |e| props.on_input.call(e.value())
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TextAreaFieldProps {
    label: String,
    placeholder: String,
    rows: i32,
    value: String,
    on_input: EventHandler<String>,
}

#[component]
fn TextAreaField(props: TextAreaFieldProps) -> Element {
    rsx! {
        div {
            label { class: LABEL, "{props.label}" }
            textarea {
                class: TEXTAREA,
                rows: "{props.rows}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                oninput: move |e| props.on_input.call(e.value())
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Success,
    Danger,
}

#[derive(Props, Clone, PartialEq)]
struct ActionButtonProps {
    text: String,
    button_type: ButtonType,
    disabled: bool,
    on_click: EventHandler<()>,
}

#[component]
fn ActionButton(props: ActionButtonProps) -> Element {
    let button_class = match props.button_type {
        ButtonType::Primary => BUTTON_PRIMARY,
        ButtonType::Success => BUTTON_SUCCESS,
        ButtonType::Danger => BUTTON_DANGER,
    };

    let full_class = if props.disabled {
        format!("{} {}", button_class, BUTTON_DISABLED)
    } else {
        button_class.to_string()
    };

    rsx! {
        button {
            class: "{full_class}",
            disabled: props.disabled,
            onclick: move |_| props.on_click.call(()),
            "{props.text}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ErrorMessageProps {
    message: String,
}

#[component]
fn ErrorMessage(props: ErrorMessageProps) -> Element {
    rsx! {
        div { class: ERROR_MESSAGE,
            "{props.message}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SuccessMessageProps {
    title: String,
    content: String,
    is_code: bool,
}

#[component]
fn SuccessMessage(props: SuccessMessageProps) -> Element {
    rsx! {
        div { class: SUCCESS_MESSAGE,
            h3 { class: SUCCESS_TITLE, "{props.title}" }
            if props.is_code {
                pre { class: CODE_BLOCK, "{props.content}" }
            } else {
                p { class: TEXT_CONTENT, "{props.content}" }
            }
        }
    }
}

// Validation Functions

fn validate_encryption_inputs(
    title: &str,
    message: &str,
    recipient_key: &str,
    private_key: &str,
    public_key: &str,
) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Title is required".to_string());
    }
    if message.trim().is_empty() {
        return Err("Message is required".to_string());
    }
    if recipient_key.trim().is_empty() {
        return Err("Recipient public key is required".to_string());
    }
    if private_key.trim().is_empty() {
        return Err("Private key is required".to_string());
    }
    if public_key.trim().is_empty() {
        return Err("Public key is required".to_string());
    }

    // Validate key formats
    if let Err(e) = SolanaEncryption::validate_public_key(recipient_key) {
        return Err(format!("Invalid recipient public key: {}", e));
    }
    if let Err(e) = SolanaEncryption::validate_private_key(private_key) {
        return Err(format!("Invalid private key: {}", e));
    }
    if let Err(e) = SolanaEncryption::validate_public_key(public_key) {
        return Err(format!("Invalid public key: {}", e));
    }

    Ok(())
}

fn validate_decryption_inputs(payload: &str, private_key: &str) -> Result<(), String> {
    if payload.trim().is_empty() {
        return Err("Encrypted payload is required".to_string());
    }
    if private_key.trim().is_empty() {
        return Err("Private key is required".to_string());
    }

    // Validate private key format
    if let Err(e) = SolanaEncryption::validate_private_key(private_key) {
        return Err(format!("Invalid private key: {}", e));
    }

    Ok(())
}
