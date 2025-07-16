use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadingState {
    pub global_loading: bool,
    pub task_loading: HashMap<String, bool>,
    pub loading_messages: HashMap<String, String>,
    pub last_updated: std::time::SystemTime,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            global_loading: false,
            task_loading: HashMap::new(),
            loading_messages: HashMap::new(),
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Loading signal manager for managing loading states
pub struct LoadingManager {
    loading_state: AsyncSignalManager<LoadingState>,
}

impl LoadingManager {
    pub fn new() -> Self {
        Self {
            loading_state: AsyncSignalManager::new(LoadingState::default()),
        }
    }

    pub fn set_global_loading(&mut self, loading: bool) {
        let mut state = self.loading_state.get().value;
        state.global_loading = loading;
        state.last_updated = std::time::SystemTime::now();
        self.loading_state.set_success(state);
    }

    pub fn set_task_loading(&mut self, task_id: &str, loading: bool) {
        let mut state = self.loading_state.get().value;
        if loading {
            state.task_loading.insert(task_id.to_string(), true);
        } else {
            state.task_loading.remove(task_id);
        }
        state.last_updated = std::time::SystemTime::now();
        self.loading_state.set_success(state);
    }

    pub fn set_loading_message(&mut self, task_id: &str, message: String) {
        let mut state = self.loading_state.get().value;
        state.loading_messages.insert(task_id.to_string(), message);
        state.last_updated = std::time::SystemTime::now();
        self.loading_state.set_success(state);
    }

    pub fn clear_loading_message(&mut self, task_id: &str) {
        let mut state = self.loading_state.get().value;
        state.loading_messages.remove(task_id);
        state.last_updated = std::time::SystemTime::now();
        self.loading_state.set_success(state);
    }

    pub fn get_loading_state(&self) -> LoadingState {
        self.loading_state.get().value
    }

    pub fn is_global_loading(&self) -> bool {
        self.loading_state.get().value.global_loading
    }

    pub fn is_task_loading(&self, task_id: &str) -> bool {
        self.loading_state.get().value.task_loading.get(task_id).copied().unwrap_or(false)
    }

    pub fn get_loading_message(&self, task_id: &str) -> Option<String> {
        self.loading_state.get().value.loading_messages.get(task_id).cloned()
    }

    pub fn get_loading_tasks(&self) -> HashMap<String, bool> {
        self.loading_state.get().value.task_loading
    }

    pub fn get_loading_messages(&self) -> HashMap<String, String> {
        self.loading_state.get().value.loading_messages
    }

    pub fn subscribe_loading(&self) -> Signal<SignalState<LoadingState>> {
        self.loading_state.subscribe()
    }

    pub fn clear_all_loading(&mut self) {
        self.loading_state.set_success(LoadingState::default());
    }
}

// Global loading manager instance
pub static LOADING_MANAGER: Signal<LoadingManager> = Signal::new(LoadingManager::new);

// Convenience functions for global access
pub fn set_global_loading(loading: bool) {
    let mut manager = LOADING_MANAGER.read().clone();
    manager.set_global_loading(loading);
    LOADING_MANAGER.set(manager);
}

pub fn set_task_loading(task_id: &str, loading: bool) {
    let mut manager = LOADING_MANAGER.read().clone();
    manager.set_task_loading(task_id, loading);
    LOADING_MANAGER.set(manager);
}

pub fn set_loading_message(task_id: &str, message: String) {
    let mut manager = LOADING_MANAGER.read().clone();
    manager.set_loading_message(task_id, message);
    LOADING_MANAGER.set(manager);
}

pub fn clear_loading_message(task_id: &str) {
    let mut manager = LOADING_MANAGER.read().clone();
    manager.clear_loading_message(task_id);
    LOADING_MANAGER.set(manager);
}

pub fn get_loading_state() -> LoadingState {
    LOADING_MANAGER.read().get_loading_state()
}

pub fn is_global_loading() -> bool {
    LOADING_MANAGER.read().is_global_loading()
}

pub fn is_task_loading(task_id: &str) -> bool {
    LOADING_MANAGER.read().is_task_loading(task_id)
}

pub fn get_loading_message(task_id: &str) -> Option<String> {
    LOADING_MANAGER.read().get_loading_message(task_id)
}

pub fn get_loading_tasks() -> HashMap<String, bool> {
    LOADING_MANAGER.read().get_loading_tasks()
}

pub fn get_loading_messages() -> HashMap<String, String> {
    LOADING_MANAGER.read().get_loading_messages()
}

pub fn subscribe_loading() -> Signal<SignalState<LoadingState>> {
    LOADING_MANAGER.read().subscribe_loading()
}

pub fn clear_all_loading() {
    let mut manager = LOADING_MANAGER.read().clone();
    manager.clear_all_loading();
    LOADING_MANAGER.set(manager);
} 