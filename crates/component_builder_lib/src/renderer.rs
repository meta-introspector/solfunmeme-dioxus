use dioxus::prelude::*;
use component_registry_lib::{ComponentName, ComponentInstance, component_name};
use component_emoji_lib::get_emoji;

#[component]
pub fn RenderComponent(instance: ComponentInstance) -> Element {
    match instance.name {
        ComponentName::MainApp => rsx! { div { "MainApp (Placeholder)" } },
        _ => {
            rsx! { 
                div { 
                    class: "border border-gray-200 rounded-lg p-4 bg-gray-50",
                    div { class: "text-sm text-gray-600", "Component: {component_name(&instance.name)}" }
                    div { class: "text-2xl mt-2", "{get_emoji(&instance.name, Some(&instance.props))}" }
                    div { class: "text-xs text-gray-500 mt-1", "ID: {instance.id}" }
                } 
            }
        }
    }
} 