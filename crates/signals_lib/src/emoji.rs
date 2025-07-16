use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiRegistry {
    pub emoji_mappings: HashMap<String, EmojiInfo>,
    pub emoji_categories: HashMap<String, Vec<String>>,
    pub dynamic_emojis: HashMap<String, DynamicEmojiInfo>,
    pub emoji_styles: HashMap<String, EmojiStyle>,
    pub emoji_hooks: HashMap<String, Vec<EmojiHook>>,
    pub active_theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiInfo {
    pub id: String,
    pub emoji: String,
    pub name: String,
    pub category: String,
    pub keywords: Vec<String>,
    pub is_enabled: bool,
    pub is_custom: bool,
    pub metadata: HashMap<String, String>,
    pub created_at: std::time::SystemTime,
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicEmojiInfo {
    pub id: String,
    pub emoji: String,
    pub name: String,
    pub source_code: String,
    pub is_compiled: bool,
    pub compilation_error: Option<String>,
    pub hot_reload_enabled: bool,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiStyle {
    pub id: String,
    pub name: String,
    pub css_classes: Vec<String>,
    pub inline_styles: HashMap<String, String>,
    pub animations: Vec<EmojiAnimation>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiAnimation {
    pub name: String,
    pub duration: f32,
    pub easing: String,
    pub keyframes: Vec<AnimationKeyframe>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationKeyframe {
    pub percentage: f32,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiHook {
    pub hook_type: EmojiHookType,
    pub target_emoji: String,
    pub aspect: EmojiAspectInfo,
    pub priority: u32,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmojiHookType {
    BeforeRender,
    AfterRender,
    OnHover,
    OnClick,
    OnLoad,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiAspectInfo {
    pub name: String,
    pub description: String,
    pub code: String,
    pub metadata: HashMap<String, String>,
}

impl Default for EmojiRegistry {
    fn default() -> Self {
        Self {
            emoji_mappings: HashMap::new(),
            emoji_categories: HashMap::new(),
            dynamic_emojis: HashMap::new(),
            emoji_styles: HashMap::new(),
            emoji_hooks: HashMap::new(),
            active_theme: "default".to_string(),
        }
    }
}

/// Emoji signal manager for dynamic emoji management
pub struct EmojiManager {
    emoji_registry: AsyncSignalManager<EmojiRegistry>,
    loading_state: AsyncSignalManager<bool>,
}

impl EmojiManager {
    pub fn new() -> Self {
        Self {
            emoji_registry: AsyncSignalManager::new(EmojiRegistry::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    // Static emoji management
    pub fn register_emoji(&mut self, emoji_info: EmojiInfo) {
        let mut registry = self.emoji_registry.get().value;
        registry.emoji_mappings.insert(emoji_info.id.clone(), emoji_info);
        self.emoji_registry.set_success(registry);
    }

    pub fn unregister_emoji(&mut self, emoji_id: &str) {
        let mut registry = self.emoji_registry.get().value;
        registry.emoji_mappings.remove(emoji_id);
        registry.emoji_hooks.remove(emoji_id);
        self.emoji_registry.set_success(registry);
    }

    pub fn enable_emoji(&mut self, emoji_id: &str) {
        let mut registry = self.emoji_registry.get().value;
        if let Some(emoji) = registry.emoji_mappings.get_mut(emoji_id) {
            emoji.is_enabled = true;
            emoji.last_updated = std::time::SystemTime::now();
        }
        self.emoji_registry.set_success(registry);
    }

    pub fn disable_emoji(&mut self, emoji_id: &str) {
        let mut registry = self.emoji_registry.get().value;
        if let Some(emoji) = registry.emoji_mappings.get_mut(emoji_id) {
            emoji.is_enabled = false;
            emoji.last_updated = std::time::SystemTime::now();
        }
        self.emoji_registry.set_success(registry);
    }

    // Dynamic emoji management
    pub fn register_dynamic_emoji(&mut self, dynamic_emoji: DynamicEmojiInfo) {
        let mut registry = self.emoji_registry.get().value;
        registry.dynamic_emojis.insert(dynamic_emoji.id.clone(), dynamic_emoji);
        self.emoji_registry.set_success(registry);
    }

    pub fn update_dynamic_emoji(&mut self, emoji_id: &str, source_code: String) {
        let mut registry = self.emoji_registry.get().value;
        if let Some(emoji) = registry.dynamic_emojis.get_mut(emoji_id) {
            emoji.source_code = source_code;
            emoji.is_compiled = false;
            emoji.compilation_error = None;
        }
        self.emoji_registry.set_success(registry);
    }

    // Emoji styles
    pub fn register_emoji_style(&mut self, style: EmojiStyle) {
        let mut registry = self.emoji_registry.get().value;
        registry.emoji_styles.insert(style.id.clone(), style);
        self.emoji_registry.set_success(registry);
    }

    pub fn set_active_theme(&mut self, theme: String) {
        let mut registry = self.emoji_registry.get().value;
        registry.active_theme = theme;
        self.emoji_registry.set_success(registry);
    }

    // Emoji categories
    pub fn create_emoji_category(&mut self, category_name: String, emoji_ids: Vec<String>) {
        let mut registry = self.emoji_registry.get().value;
        registry.emoji_categories.insert(category_name, emoji_ids);
        self.emoji_registry.set_success(registry);
    }

    pub fn add_to_emoji_category(&mut self, category_name: &str, emoji_id: String) {
        let mut registry = self.emoji_registry.get().value;
        let category = registry.emoji_categories.entry(category_name.to_string()).or_insert_with(Vec::new);
        if !category.contains(&emoji_id) {
            category.push(emoji_id);
        }
        self.emoji_registry.set_success(registry);
    }

    // AOP hooks
    pub fn register_emoji_hook(&mut self, emoji_id: &str, hook: EmojiHook) {
        let mut registry = self.emoji_registry.get().value;
        let hooks = registry.emoji_hooks.entry(emoji_id.to_string()).or_insert_with(Vec::new);
        hooks.push(hook);
        // Sort by priority
        hooks.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.emoji_registry.set_success(registry);
    }

    pub fn unregister_emoji_hook(&mut self, emoji_id: &str, hook_name: &str) {
        let mut registry = self.emoji_registry.get().value;
        if let Some(hooks) = registry.emoji_hooks.get_mut(emoji_id) {
            hooks.retain(|hook| hook.aspect.name != hook_name);
        }
        self.emoji_registry.set_success(registry);
    }

    // Getters
    pub fn get_emoji(&self, emoji_id: &str) -> Option<EmojiInfo> {
        self.emoji_registry.get().value.emoji_mappings.get(emoji_id).cloned()
    }

    pub fn get_emoji_by_name(&self, name: &str) -> Option<EmojiInfo> {
        self.emoji_registry.get().value.emoji_mappings.values()
            .find(|e| e.name == name && e.is_enabled)
            .cloned()
    }

    pub fn get_enabled_emojis(&self) -> Vec<EmojiInfo> {
        self.emoji_registry.get().value.emoji_mappings.values()
            .filter(|e| e.is_enabled)
            .cloned()
            .collect()
    }

    pub fn get_emojis_by_category(&self, category: &str) -> Vec<EmojiInfo> {
        self.emoji_registry.get().value.emoji_mappings.values()
            .filter(|e| e.category == category && e.is_enabled)
            .cloned()
            .collect()
    }

    pub fn search_emojis(&self, query: &str) -> Vec<EmojiInfo> {
        let query_lower = query.to_lowercase();
        self.emoji_registry.get().value.emoji_mappings.values()
            .filter(|e| e.is_enabled && (
                e.name.to_lowercase().contains(&query_lower) ||
                e.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            ))
            .cloned()
            .collect()
    }

    pub fn get_emoji_style(&self, style_id: &str) -> Option<EmojiStyle> {
        self.emoji_registry.get().value.emoji_styles.get(style_id).cloned()
    }

    pub fn get_active_theme(&self) -> String {
        self.emoji_registry.get().value.active_theme.clone()
    }

    pub fn get_emoji_hooks(&self, emoji_id: &str) -> Vec<EmojiHook> {
        self.emoji_registry.get().value.emoji_hooks.get(emoji_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_emoji_registry(&self) -> EmojiRegistry {
        self.emoji_registry.get().value
    }

    pub fn subscribe_emoji(&self) -> Signal<SignalState<EmojiRegistry>> {
        self.emoji_registry.subscribe()
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

// Global emoji manager instance
pub static EMOJI_MANAGER: Signal<EmojiManager> = Signal::new(EmojiManager::new);

// Convenience functions
pub fn register_emoji(emoji_info: EmojiInfo) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.register_emoji(emoji_info);
    EMOJI_MANAGER.set(manager);
}

pub fn unregister_emoji(emoji_id: &str) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.unregister_emoji(emoji_id);
    EMOJI_MANAGER.set(manager);
}

pub fn enable_emoji(emoji_id: &str) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.enable_emoji(emoji_id);
    EMOJI_MANAGER.set(manager);
}

pub fn disable_emoji(emoji_id: &str) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.disable_emoji(emoji_id);
    EMOJI_MANAGER.set(manager);
}

pub fn register_dynamic_emoji(dynamic_emoji: DynamicEmojiInfo) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.register_dynamic_emoji(dynamic_emoji);
    EMOJI_MANAGER.set(manager);
}

pub fn update_dynamic_emoji(emoji_id: &str, source_code: String) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.update_dynamic_emoji(emoji_id, source_code);
    EMOJI_MANAGER.set(manager);
}

pub fn register_emoji_style(style: EmojiStyle) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.register_emoji_style(style);
    EMOJI_MANAGER.set(manager);
}

pub fn set_active_emoji_theme(theme: String) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.set_active_theme(theme);
    EMOJI_MANAGER.set(manager);
}

pub fn register_emoji_hook(emoji_id: &str, hook: EmojiHook) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.register_emoji_hook(emoji_id, hook);
    EMOJI_MANAGER.set(manager);
}

pub fn get_emoji(emoji_id: &str) -> Option<EmojiInfo> {
    EMOJI_MANAGER.read().get_emoji(emoji_id)
}

pub fn get_emoji_by_name(name: &str) -> Option<EmojiInfo> {
    EMOJI_MANAGER.read().get_emoji_by_name(name)
}

pub fn get_enabled_emojis() -> Vec<EmojiInfo> {
    EMOJI_MANAGER.read().get_enabled_emojis()
}

pub fn get_emojis_by_category(category: &str) -> Vec<EmojiInfo> {
    EMOJI_MANAGER.read().get_emojis_by_category(category)
}

pub fn search_emojis(query: &str) -> Vec<EmojiInfo> {
    EMOJI_MANAGER.read().search_emojis(query)
}

pub fn get_emoji_style(style_id: &str) -> Option<EmojiStyle> {
    EMOJI_MANAGER.read().get_emoji_style(style_id)
}

pub fn get_active_emoji_theme() -> String {
    EMOJI_MANAGER.read().get_active_theme()
}

pub fn subscribe_emoji() -> Signal<SignalState<EmojiRegistry>> {
    EMOJI_MANAGER.read().subscribe_emoji()
}

pub fn set_emoji_loading(loading: bool) {
    let mut manager = EMOJI_MANAGER.read().clone();
    manager.set_loading(loading);
    EMOJI_MANAGER.set(manager);
}

pub fn is_emoji_loading() -> bool {
    EMOJI_MANAGER.read().is_loading()
} 