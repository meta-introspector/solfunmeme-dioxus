use dioxus::prelude::*;
use std::collections::HashMap;

/// Simple interface that all dynamic modules must implement
pub trait ModuleInterface: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn render(&self) -> Element;
    fn initialize(&mut self) -> Result<(), String> {
        Ok(()) // Default implementation
    }
    fn shutdown(&mut self) {
        // Default implementation
    }
}

/// Opaque wrapper around modules
pub struct ModuleWrapper {
    inner: Box<dyn ModuleInterface>,
}

impl ModuleWrapper {
    pub fn new<T: ModuleInterface + 'static>(module: T) -> Self {
        Self {
            inner: Box::new(module),
        }
    }

    pub fn name(&self) -> &str {
        self.inner.name()
    }

    pub fn version(&self) -> &str {
        self.inner.version()
    }

    pub fn render(&self) -> Element {
        self.inner.render()
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.inner.initialize()
    }

    pub fn shutdown(&mut self) {
        self.inner.shutdown();
    }
}

/// Registry for managing dynamic modules
pub struct ModuleRegistry {
    modules: HashMap<String, ModuleWrapper>,
    loaded_modules: Signal<Vec<String>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            loaded_modules: Signal::new(Vec::new()),
        }
    }

    pub fn register_module(&mut self, name: String, module: ModuleWrapper) -> Result<(), String> {
        let mut module = module;
        module.initialize()?;
        
        self.modules.insert(name.clone(), module);
        self.update_loaded_modules();
        Ok(())
    }

    pub fn unregister_module(&mut self, name: &str) {
        if let Some(mut module) = self.modules.remove(name) {
            module.shutdown();
            self.update_loaded_modules();
        }
    }

    pub fn get_module(&self, name: &str) -> Option<&ModuleWrapper> {
        self.modules.get(name)
    }

    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    pub fn subscribe_loaded_modules(&self) -> Signal<Vec<String>> {
        self.loaded_modules.clone()
    }

    fn update_loaded_modules(&mut self) {
        let modules = self.modules.keys().cloned().collect();
        self.loaded_modules.set(modules);
    }
}

/// Example module implementation
pub struct ExampleModule {
    name: String,
    version: String,
}

impl ExampleModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "1.0.0".to_string(),
        }
    }
}

impl ModuleInterface for ExampleModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn render(&self) -> Element {
        rsx! {
            div {
                class: "example-module",
                h3 { "Module: {self.name}" }
                p { "Version: {self.version}" }
                p { "This is a dynamically loaded module!" }
            }
        }
    }
}

/// Hook for using the module registry in components
pub fn use_module_registry() -> Option<Signal<ModuleRegistry>> {
    Some(use_context::<Signal<ModuleRegistry>>())
}

/// Component for displaying loaded modules
#[component]
pub fn ModuleList() -> Element {
    let registry = use_module_registry();
    
    if let Some(registry) = registry {
        let loaded_modules = registry.read().subscribe_loaded_modules();
        
        rsx! {
            div {
                class: "module-list",
                h2 { "Loaded Modules" }
                for module_name in loaded_modules.read().iter() {
                    div {
                        key: "{module_name}",
                        class: "module-item",
                        p { "Module: {module_name}" }
                        if let Some(module) = registry.read().get_module(module_name) {
                            {module.render()}
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                p { "Module registry not available" }
            }
        }
    }
} 