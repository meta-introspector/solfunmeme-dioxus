use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRegistry {
    pub components: HashMap<String, ComponentInfo>,
    pub component_groups: HashMap<String, Vec<String>>,
    pub dynamic_components: HashMap<String, DynamicComponentInfo>,
    pub component_hooks: HashMap<String, Vec<ComponentHook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub is_enabled: bool,
    pub is_dynamic: bool,
    pub props_schema: Option<PropsSchema>,
    pub metadata: HashMap<String, String>,
    pub created_at: std::time::SystemTime,
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicComponentInfo {
    pub id: String,
    pub name: String,
    pub source_code: String,
    pub compiled_wasm: Option<Vec<u8>>,
    pub dependencies: Vec<String>,
    pub is_compiled: bool,
    pub compilation_error: Option<String>,
    pub hot_reload_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropsSchema {
    pub properties: HashMap<String, PropDefinition>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDefinition {
    pub prop_type: PropType,
    pub default_value: Option<String>,
    pub description: String,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Function,
    Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHook {
    pub hook_type: HookType,
    pub target_component: String,
    pub aspect: AspectInfo,
    pub priority: u32,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HookType {
    BeforeRender,
    AfterRender,
    BeforeMount,
    AfterMount,
    BeforeUnmount,
    AfterUnmount,
    BeforeUpdate,
    AfterUpdate,
    ErrorBoundary,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectInfo {
    pub name: String,
    pub description: String,
    pub code: String,
    pub metadata: HashMap<String, String>,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            component_groups: HashMap::new(),
            dynamic_components: HashMap::new(),
            component_hooks: HashMap::new(),
        }
    }
}

/// Component signal manager for dynamic component registration
pub struct ComponentManager {
    component_registry: AsyncSignalManager<ComponentRegistry>,
    loading_state: AsyncSignalManager<bool>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_registry: AsyncSignalManager::new(ComponentRegistry::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    // Static component management
    pub fn register_component(&mut self, component: ComponentInfo) {
        let mut registry = self.component_registry.get().value;
        registry.components.insert(component.id.clone(), component);
        self.component_registry.set_success(registry);
    }

    pub fn unregister_component(&mut self, component_id: &str) {
        let mut registry = self.component_registry.get().value;
        registry.components.remove(component_id);
        registry.component_hooks.remove(component_id);
        self.component_registry.set_success(registry);
    }

    pub fn enable_component(&mut self, component_id: &str) {
        let mut registry = self.component_registry.get().value;
        if let Some(component) = registry.components.get_mut(component_id) {
            component.is_enabled = true;
            component.last_updated = std::time::SystemTime::now();
        }
        self.component_registry.set_success(registry);
    }

    pub fn disable_component(&mut self, component_id: &str) {
        let mut registry = self.component_registry.get().value;
        if let Some(component) = registry.components.get_mut(component_id) {
            component.is_enabled = false;
            component.last_updated = std::time::SystemTime::now();
        }
        self.component_registry.set_success(registry);
    }

    // Dynamic component management
    pub fn register_dynamic_component(&mut self, dynamic_component: DynamicComponentInfo) {
        let mut registry = self.component_registry.get().value;
        registry.dynamic_components.insert(dynamic_component.id.clone(), dynamic_component);
        self.component_registry.set_success(registry);
    }

    pub fn update_dynamic_component(&mut self, component_id: &str, source_code: String) {
        let mut registry = self.component_registry.get().value;
        if let Some(component) = registry.dynamic_components.get_mut(component_id) {
            component.source_code = source_code;
            component.is_compiled = false;
            component.compilation_error = None;
        }
        self.component_registry.set_success(registry);
    }

    pub fn set_compilation_status(&mut self, component_id: &str, is_compiled: bool, error: Option<String>) {
        let mut registry = self.component_registry.get().value;
        if let Some(component) = registry.dynamic_components.get_mut(component_id) {
            component.is_compiled = is_compiled;
            component.compilation_error = error;
        }
        self.component_registry.set_success(registry);
    }

    // Component groups
    pub fn create_component_group(&mut self, group_name: String, component_ids: Vec<String>) {
        let mut registry = self.component_registry.get().value;
        registry.component_groups.insert(group_name, component_ids);
        self.component_registry.set_success(registry);
    }

    pub fn add_to_group(&mut self, group_name: &str, component_id: String) {
        let mut registry = self.component_registry.get().value;
        let group = registry.component_groups.entry(group_name.to_string()).or_insert_with(Vec::new);
        if !group.contains(&component_id) {
            group.push(component_id);
        }
        self.component_registry.set_success(registry);
    }

    // AOP hooks
    pub fn register_hook(&mut self, component_id: &str, hook: ComponentHook) {
        let mut registry = self.component_registry.get().value;
        let hooks = registry.component_hooks.entry(component_id.to_string()).or_insert_with(Vec::new);
        hooks.push(hook);
        // Sort by priority
        hooks.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.component_registry.set_success(registry);
    }

    pub fn unregister_hook(&mut self, component_id: &str, hook_name: &str) {
        let mut registry = self.component_registry.get().value;
        if let Some(hooks) = registry.component_hooks.get_mut(component_id) {
            hooks.retain(|hook| hook.aspect.name != hook_name);
        }
        self.component_registry.set_success(registry);
    }

    // Getters
    pub fn get_component(&self, component_id: &str) -> Option<ComponentInfo> {
        self.component_registry.get().value.components.get(component_id).cloned()
    }

    pub fn get_dynamic_component(&self, component_id: &str) -> Option<DynamicComponentInfo> {
        self.component_registry.get().value.dynamic_components.get(component_id).cloned()
    }

    pub fn get_enabled_components(&self) -> Vec<ComponentInfo> {
        self.component_registry.get().value.components.values()
            .filter(|c| c.is_enabled)
            .cloned()
            .collect()
    }

    pub fn get_components_by_category(&self, category: &str) -> Vec<ComponentInfo> {
        self.component_registry.get().value.components.values()
            .filter(|c| c.category == category && c.is_enabled)
            .cloned()
            .collect()
    }

    pub fn get_component_hooks(&self, component_id: &str) -> Vec<ComponentHook> {
        self.component_registry.get().value.component_hooks.get(component_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_component_registry(&self) -> ComponentRegistry {
        self.component_registry.get().value
    }

    pub fn subscribe_components(&self) -> Signal<SignalState<ComponentRegistry>> {
        self.component_registry.subscribe()
    }

    // Loading state
    pub fn set_loading(&mut self, loading: bool) {
        if loading {
            self.loading_state.set_loading();
        } else {
            self.loading_state.set_success(false);
        }
    }

    pub fn is_loading(&self) -> bool {
        self.loading_state.get().value
    }

    pub fn subscribe_loading(&self) -> Signal<SignalState<bool>> {
        self.loading_state.subscribe()
    }
}

// Global component manager instance
pub static COMPONENT_MANAGER: GlobalSignal<ComponentManager> = 
    Signal::global(ComponentManager::new);

// Convenience functions
pub fn register_component(component: ComponentInfo) {
    COMPONENT_MANAGER.read().register_component(component);
}

pub fn unregister_component(component_id: &str) {
    COMPONENT_MANAGER.read().unregister_component(component_id);
}

pub fn enable_component(component_id: &str) {
    COMPONENT_MANAGER.read().enable_component(component_id);
}

pub fn disable_component(component_id: &str) {
    COMPONENT_MANAGER.read().disable_component(component_id);
}

pub fn register_dynamic_component(dynamic_component: DynamicComponentInfo) {
    COMPONENT_MANAGER.read().register_dynamic_component(dynamic_component);
}

pub fn update_dynamic_component(component_id: &str, source_code: String) {
    COMPONENT_MANAGER.read().update_dynamic_component(component_id, source_code);
}

pub fn register_hook(component_id: &str, hook: ComponentHook) {
    COMPONENT_MANAGER.read().register_hook(component_id, hook);
}

pub fn get_component(component_id: &str) -> Option<ComponentInfo> {
    COMPONENT_MANAGER.read().get_component(component_id)
}

pub fn get_enabled_components() -> Vec<ComponentInfo> {
    COMPONENT_MANAGER.read().get_enabled_components()
}

pub fn get_components_by_category(category: &str) -> Vec<ComponentInfo> {
    COMPONENT_MANAGER.read().get_components_by_category(category)
}

pub fn subscribe_components() -> Signal<SignalState<ComponentRegistry>> {
    COMPONENT_MANAGER.read().subscribe_components()
}

pub fn set_component_loading(loading: bool) {
    COMPONENT_MANAGER.read().set_loading(loading);
}

pub fn is_component_loading() -> bool {
    COMPONENT_MANAGER.read().is_loading()
} 