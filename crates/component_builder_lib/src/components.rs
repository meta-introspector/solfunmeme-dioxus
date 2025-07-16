use dioxus::prelude::*;
use component_registry_lib::{ComponentName, ComponentInstance, PropValue, get_component_props};
use component_emoji_lib::{get_emoji, get_emoji_style};

#[component]
pub fn ComponentEmojiDialog(components: Vec<ComponentInstance>) -> Element {
    let mut show_emoji_dialog = use_signal(|| false);
    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div {
                class: "bg-white rounded-lg p-6 w-full max-w-md",
                h2 { class: "text-2xl font-semibold mb-4", "Emoji Composition" }
                div { class: "flex flex-wrap gap-2 mb-4",
                    for instance in components.iter() {
                        span {
                            class: format!("text-2xl {}", get_emoji_style(&instance.name, &instance.props)),
                            "{get_emoji(&instance.name, Some(&instance.props))}"
                        }
                    }
                }
                button {
                    class: "bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600",
                    onclick: move |_| show_emoji_dialog.set(false),
                    "Close"
                }
            }
        }
    }
}

#[component]
pub fn ComponentConfigPanel(
    component: ComponentName,
    on_update: EventHandler<(String, PropValue)>,
) -> Element {
    let props = get_component_props(&component);
    rsx! {
        div { class: "border border-gray-200 rounded-lg p-4",
            h3 { class: "font-medium text-lg mb-2", "Configure Properties" }
            for prop in props {
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700", "{prop.0}" }
                    match prop.1 {
                        "bool" => {
                            let mut value = use_signal(|| false);
                            rsx! {
                                input {
                                    r#type: "checkbox",
                                    checked: "{value}",
                                    onchange: move |evt| {
                                        value.set(evt.checked());
                                        on_update.call((prop.0.to_string(), PropValue::Bool(evt.checked())));
                                    }
                                }
                            }
                        }
                        "string" => {
                            let mut value = use_signal(|| String::new());
                            rsx! {
                                input {
                                    class: "w-full p-2 border border-gray-300 rounded-lg",
                                    value: "{value}",
                                    oninput: move |evt| {
                                        value.set(evt.value().clone());
                                        on_update.call((prop.0.to_string(), PropValue::String(evt.value().clone())));
                                    }
                                }
                            }
                        }
                        _ => rsx! { span { class: "text-sm text-gray-500", "Unsupported prop type" } },
                    }
                }
            }
        }
    }
} 