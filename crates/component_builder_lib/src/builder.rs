use dioxus::prelude::*;
use component_registry_lib::{ComponentName, ComponentInstance, PropValue, component_name, get_component_categories};
use component_emoji_lib::get_emoji;
use std::collections::HashMap;
use crate::components::{ComponentConfigPanel, ComponentEmojiDialog};
use crate::renderer::RenderComponent;

#[component]
pub fn ComponentBuilderEmojiApp() -> Element {
    let mut selected_component = use_signal(|| None::<ComponentName>);
    let mut components = use_signal(|| vec![] as Vec<ComponentInstance>);
    let mut next_id = use_signal(|| 0u32);
    let mut props_config = use_signal(|| HashMap::<String, PropValue>::new());
    let mut show_emoji_dialog = use_signal(|| false);

    let update_prop = Callback::new(move |(key, value): (String, PropValue)| {
        props_config.write().insert(key, value);
    });

    let mut add_component = move || {
        let name = {
            selected_component.read().clone()
        };
        if let Some(name) = name {
            let id = *next_id.read();
            let instance = ComponentInstance {
                name: name.clone(),
                props: props_config.read().clone(),
                id,
            };
            components.write().push(instance);
            next_id.set(id + 1);
            props_config.set(HashMap::new()); // Reset props
            selected_component.set(None);
        }
    };

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6 text-center",
                "Component Builder (Emoji Edition)"
            }
            div { class: "mb-4",
                button {
                    class: "bg-purple-500 text-white px-4 py-2 rounded-lg hover:bg-purple-600",
                    onclick: move |_| show_emoji_dialog.set(!show_emoji_dialog()),
                    "Toggle Emoji Dialog"
                }
            }
            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Component Selection Sidebar
                div { class: "lg:col-span-1",
                    div { class: "bg-white shadow-lg rounded-lg p-4",
                        h2 { class: "text-xl font-semibold mb-4", "Select Component" }
                        for category in get_component_categories() {
                            div { class: "mb-4",
                                h3 { class: "text-lg font-medium mb-2", "{category.0}" }
                                for component in category.1 {
                                    button {
                                        class: format!(
                                            "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
                                            if selected_component() == Some(component.clone()) {
                                                "bg-blue-500 text-white"
                                            } else {
                                                "bg-gray-100 hover:bg-gray-200"
                                            }
                                        ),
                                        onclick: move |_| selected_component.set(Some(component.clone())),
                                        "{component_name(&component)}"
                                        span { class: "ml-2", "{get_emoji(&component, None)}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Configuration and Preview Panel
                div { class: "lg:col-span-3",
                    div { class: "bg-white shadow-lg rounded-lg p-6",
                        div { class: "flex justify-between items-center mb-4",
                            h2 { class: "text-2xl font-semibold",
                                "{selected_component().map(|c| component_name(&c)).unwrap_or_default()}"
                            }
                            button {
                                class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
                                onclick: move |_| add_component(),
                                "Add Component"
                            }
                        }
                        if let Some(component) = selected_component() {
                            ComponentConfigPanel {
                                component,
                                on_update: update_prop
                            }
                        }
                        div { class: "mt-6",
                            h3 { class: "text-lg font-semibold mb-2", "Composed Components" }
                            div { class: "grid gap-4",
                                for instance in components.read().iter() {
                                    RenderComponent { instance: instance.clone() }
                                }
                            }
                        }
                    }
                }
            }
            if *show_emoji_dialog.read() {
                ComponentEmojiDialog {
                    components: components.read().clone(),
                }
            }
        }
    }
} 