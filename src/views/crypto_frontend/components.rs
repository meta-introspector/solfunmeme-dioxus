use crate::views::crypto_style::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    pub(crate) title: String,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        h2 { class: CARD_TITLE, "{props.title}" }
    }
}
#[allow(dead_code)]
const FORM_GROUP: &str = "mb-4";
#[allow(dead_code)]
const FORM_LABEL: &str = "block text-sm font-medium text-gray-700 mb-1";
#[allow(dead_code)]
const FORM_INPUT: &str = "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500";
#[allow(dead_code)]
const FORM_TEXTAREA: &str = "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500";

#[derive(Props, Clone, PartialEq)]
pub struct InputFieldProps {
    pub label: String,
    pub placeholder: String,
    pub input_type: String,
    pub value: String,
    pub on_input: EventHandler<String>,
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    rsx! {
        div { class: FORM_GROUP,
            label { class: FORM_LABEL, "{props.label}" }
            input {
                class: FORM_INPUT,
                r#type: "{props.input_type}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                oninput: move |e| props.on_input.call(e.value().clone())
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TextAreaFieldProps {
    label: String,
    placeholder: String,
    rows: i32,
    value: String,
    on_input: EventHandler<String>,
}

#[component]
pub fn TextAreaField(props: TextAreaFieldProps) -> Element {
    rsx! {
        div { class: FORM_GROUP,
            label { class: FORM_LABEL, "{props.label}" }
            textarea {
                class: FORM_TEXTAREA,
                rows: "{props.rows}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                oninput: move |e| props.on_input.call(e.value().clone())
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ButtonType {
    #[allow(dead_code)]
    Primary,
    #[allow(dead_code)]
    Success,
    #[allow(dead_code)]
    Danger,
}

#[derive(Props, Clone, PartialEq)]
pub struct ActionButtonProps {
    pub text: String,
    pub button_type: ButtonType,
    pub disabled: bool,
    pub on_click: EventHandler<()>,
}

#[component]
pub fn ActionButton(props: ActionButtonProps) -> Element {
    let button_class = match props.button_type {
        ButtonType::Primary => BUTTON_PRIMARY,
        ButtonType::Success => BUTTON_SUCCESS,
        ButtonType::Danger => BUTTON_DANGER,
    };

    rsx! {
        button {
            class: "{button_class}",
            disabled: props.disabled,
            onclick: move |_| props.on_click.call(()),
            "{props.text}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ErrorMessageProps {
    pub message: String,
}

#[component]
pub fn ErrorMessage(props: ErrorMessageProps) -> Element {
    rsx! {
        div { class: ERROR_MESSAGE,
            "{props.message}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SuccessMessageProps {
    pub title: String,
    pub content: String,
    pub is_code: bool,
}

#[component]
pub fn SuccessMessage(props: SuccessMessageProps) -> Element {
    rsx! {
        div { class: SUCCESS_MESSAGE,
            h3 { "{props.title}" }
            if props.is_code {
                pre { class: CODE_BLOCK,
                    "{props.content}"
                }
            } else {
                p { "{props.content}" }
            }
        }
    }
}
