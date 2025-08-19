use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shared_types_lib::MenuOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuRegistry {
    pub menu_items: HashMap<String, MenuItem>,
    pub menu_groups: HashMap<String, Vec<String>>,
    pub dynamic_menus: HashMap<String, DynamicMenuInfo>,
    pub menu_hooks: HashMap<String, Vec<MenuHook>>,
    pub active_menu: Option<String>,
    pub menu_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub icon: Option<String>,
    pub emoji: Option<String>,
    pub is_enabled: bool,
    pub is_visible: bool,
    pub is_dynamic: bool,
    pub menu_option: MenuOption,
    pub action_type: MenuActionType,
    pub metadata: HashMap<String, String>,
    pub created_at: std::time::SystemTime,
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicMenuInfo {
    pub id: String,
    pub name: String,
    pub source_code: String,
    pub is_compiled: bool,
    pub compilation_error: Option<String>,
    pub hot_reload_enabled: bool,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuActionType {
    Navigate,
    Execute,
    Toggle,
    Modal,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuHook {
    pub hook_type: MenuHookType,
    pub target_menu: String,
    pub aspect: MenuAspectInfo,
    pub priority: u32,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuHookType {
    BeforeShow,
    AfterShow,
    BeforeHide,
    AfterHide,
    BeforeAction,
    AfterAction,
    OnClick,
    OnHover,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuAspectInfo {
    pub name: String,
    pub description: String,
    pub code: String,
    pub metadata: HashMap<String, String>,
}

impl Default for MenuRegistry {
    fn default() -> Self {
        Self {
            menu_items: HashMap::new(),
            menu_groups: HashMap::new(),
            dynamic_menus: HashMap::new(),
            menu_hooks: HashMap::new(),
            active_menu: None,
            menu_history: Vec::new(),
        }
    }
}

/// Menu signal manager for dynamic menu management
pub struct MenuManager {
    menu_registry: AsyncSignalManager<MenuRegistry>,
    loading_state: AsyncSignalManager<bool>,
}

impl MenuManager {
    pub fn new() -> Self {
        Self {
            menu_registry: AsyncSignalManager::new(MenuRegistry::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    // Static menu management
    pub fn register_menu_item(&mut self, menu_item: MenuItem) {
        let mut registry = self.menu_registry.get().value;
        registry.menu_items.insert(menu_item.id.clone(), menu_item);
        self.menu_registry.set_success(registry);
    }

    pub fn unregister_menu_item(&mut self, menu_id: &str) {
        let mut registry = self.menu_registry.get().value;
        registry.menu_items.remove(menu_id);
        registry.menu_hooks.remove(menu_id);
        self.menu_registry.set_success(registry);
    }

    pub fn enable_menu_item(&mut self, menu_id: &str) {
        let mut registry = self.menu_registry.get().value;
        if let Some(menu_item) = registry.menu_items.get_mut(menu_id) {
            menu_item.is_enabled = true;
            menu_item.last_updated = std::time::SystemTime::now();
        }
        self.menu_registry.set_success(registry);
    }

    pub fn disable_menu_item(&mut self, menu_id: &str) {
        let mut registry = self.menu_registry.get().value;
        if let Some(menu_item) = registry.menu_items.get_mut(menu_id) {
            menu_item.is_enabled = false;
            menu_item.last_updated = std::time::SystemTime::now();
        }
        self.menu_registry.set_success(registry);
    }

    pub fn show_menu_item(&mut self, menu_id: &str) {
        let mut registry = self.menu_registry.get().value;
        if let Some(menu_item) = registry.menu_items.get_mut(menu_id) {
            menu_item.is_visible = true;
            menu_item.last_updated = std::time::SystemTime::now();
        }
        self.menu_registry.set_success(registry);
    }

    pub fn hide_menu_item(&mut self, menu_id: &str) {
        let mut registry = self.menu_registry.get().value;
        if let Some(menu_item) = registry.menu_items.get_mut(menu_id) {
            menu_item.is_visible = false;
            menu_item.last_updated = std::time::SystemTime::now();
        }
        self.menu_registry.set_success(registry);
    }

    // Dynamic menu management
    pub fn register_dynamic_menu(&mut self, dynamic_menu: DynamicMenuInfo) {
        let mut registry = self.menu_registry.get().value;
        registry.dynamic_menus.insert(dynamic_menu.id.clone(), dynamic_menu);
        self.menu_registry.set_success(registry);
    }

    pub fn update_dynamic_menu(&mut self, menu_id: &str, source_code: String) {
        let mut registry = self.menu_registry.get().value;
        if let Some(menu) = registry.dynamic_menus.get_mut(menu_id) {
            menu.source_code = source_code;
            menu.is_compiled = false;
            menu.compilation_error = None;
        }
        self.menu_registry.set_success(registry);
    }

    // Menu navigation
    pub fn set_active_menu(&mut self, menu_id: &str) {
        let mut registry = self.menu_registry.get().value;
        if let Some(previous_active) = registry.active_menu.clone() {
            registry.menu_history.push(previous_active);
        }
        registry.active_menu = Some(menu_id.to_string());
        self.menu_registry.set_success(registry);
    }

    pub fn go_back(&mut self) -> Option<String> {
        let mut registry = self.menu_registry.get().value;
        let previous = registry.menu_history.pop();
        if let Some(prev) = previous {
            registry.active_menu = Some(prev.clone());
            self.menu_registry.set_success(registry);
            Some(prev)
        } else {
            None
        }
    }

    // Menu groups
    pub fn create_menu_group(&mut self, group_name: String, menu_ids: Vec<String>) {
        let mut registry = self.menu_registry.get().value;
        registry.menu_groups.insert(group_name, menu_ids);
        self.menu_registry.set_success(registry);
    }

    pub fn add_to_menu_group(&mut self, group_name: &str, menu_id: String) {
        let mut registry = self.menu_registry.get().value;
        let group = registry.menu_groups.entry(group_name.to_string()).or_insert_with(Vec::new);
        if !group.contains(&menu_id) {
            group.push(menu_id);
        }
        self.menu_registry.set_success(registry);
    }

    // AOP hooks
    pub fn register_menu_hook(&mut self, menu_id: &str, hook: MenuHook) {
        let mut registry = self.menu_registry.get().value;
        let hooks = registry.menu_hooks.entry(menu_id.to_string()).or_insert_with(Vec::new);
        hooks.push(hook);
        // Sort by priority
        hooks.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.menu_registry.set_success(registry);
    }

    pub fn unregister_menu_hook(&mut self, menu_id: &str, hook_name: &str) {
        let mut registry = self.menu_registry.get().value;
        if let Some(hooks) = registry.menu_hooks.get_mut(menu_id) {
            hooks.retain(|hook| hook.aspect.name != hook_name);
        }
        self.menu_registry.set_success(registry);
    }

    // Getters
    pub fn get_menu_item(&self, menu_id: &str) -> Option<MenuItem> {
        self.menu_registry.get().value.menu_items.get(menu_id).cloned()
    }

    pub fn get_visible_menu_items(&self) -> Vec<MenuItem> {
        self.menu_registry.get().value.menu_items.values()
            .filter(|m| m.is_visible && m.is_enabled)
            .cloned()
            .collect()
    }

    pub fn get_menu_items_by_category(&self, category: &str) -> Vec<MenuItem> {
        self.menu_registry.get().value.menu_items.values()
            .filter(|m| m.category == category && m.is_visible && m.is_enabled)
            .cloned()
            .collect()
    }

    pub fn get_active_menu(&self) -> Option<String> {
        self.menu_registry.get().value.active_menu.clone()
    }

    pub fn get_menu_history(&self) -> Vec<String> {
        self.menu_registry.get().value.menu_history.clone()
    }

    pub fn get_menu_hooks(&self, menu_id: &str) -> Vec<MenuHook> {
        self.menu_registry.get().value.menu_hooks.get(menu_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_menu_registry(&self) -> MenuRegistry {
        self.menu_registry.get().value
    }

    pub fn subscribe_menu(&self) -> Signal<SignalState<MenuRegistry>> {
        self.menu_registry.subscribe()
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

// Global menu manager instance
pub static MENU_MANAGER: GlobalSignal<MenuManager> = 
    Signal::global(MenuManager::new);

// Convenience functions
pub fn register_menu_item(menu_item: MenuItem) {
    MENU_MANAGER.read().register_menu_item(menu_item);
}

pub fn unregister_menu_item(menu_id: &str) {
    MENU_MANAGER.read().unregister_menu_item(menu_id);
}

pub fn enable_menu_item(menu_id: &str) {
    MENU_MANAGER.read().enable_menu_item(menu_id);
}

pub fn disable_menu_item(menu_id: &str) {
    MENU_MANAGER.read().disable_menu_item(menu_id);
}

pub fn show_menu_item(menu_id: &str) {
    MENU_MANAGER.read().show_menu_item(menu_id);
}

pub fn hide_menu_item(menu_id: &str) {
    MENU_MANAGER.read().hide_menu_item(menu_id);
}

pub fn set_active_menu(menu_id: &str) {
    MENU_MANAGER.read().set_active_menu(menu_id);
}

pub fn go_back_menu() -> Option<String> {
    MENU_MANAGER.read().go_back()
}

pub fn register_menu_hook(menu_id: &str, hook: MenuHook) {
    MENU_MANAGER.read().register_menu_hook(menu_id, hook);
}

pub fn get_menu_item(menu_id: &str) -> Option<MenuItem> {
    MENU_MANAGER.read().get_menu_item(menu_id)
}

pub fn get_visible_menu_items() -> Vec<MenuItem> {
    MENU_MANAGER.read().get_visible_menu_items()
}

pub fn get_menu_items_by_category(category: &str) -> Vec<MenuItem> {
    MENU_MANAGER.read().get_menu_items_by_category(category)
}

pub fn get_active_menu() -> Option<String> {
    MENU_MANAGER.read().get_active_menu()
}

pub fn subscribe_menu() -> Signal<SignalState<MenuRegistry>> {
    MENU_MANAGER.read().subscribe_menu()
}

pub fn set_menu_loading(loading: bool) {
    MENU_MANAGER.read().set_loading(loading);
}

pub fn is_menu_loading() -> bool {
    MENU_MANAGER.read().is_loading()
} 