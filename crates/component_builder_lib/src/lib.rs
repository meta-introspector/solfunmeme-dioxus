// Re-export main components
pub use crate::builder::ComponentBuilderEmojiApp;
pub use crate::components::{ComponentConfigPanel, ComponentEmojiDialog};
pub use crate::renderer::RenderComponent;

mod builder;
mod components;
mod renderer;

use dioxus::prelude::*;
use signals_lib::module_interface::{ModuleInterface, ModuleWrapper};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// Example dynamic module for component building
pub struct ComponentBuilderModule {
    name: String,
    version: String,
}

impl ComponentBuilderModule {
    pub fn new() -> Self {
        Self {
            name: "Component Builder".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

impl ModuleInterface for ComponentBuilderModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn render(&self) -> Element {
        rsx! {
            div {
                class: "component-builder-module",
                h3 { "Component Builder Module" }
                p { "This module provides component building capabilities" }
                button {
                    onclick: move |_| {
                        println!("Component builder module activated!");
                    },
                    "Build Component"
                }
            }
        }
    }
}

/// Function to create a module wrapper for this crate
pub fn create_module() -> ModuleWrapper {
    ModuleWrapper::new(ComponentBuilderModule::new())
}
