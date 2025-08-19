use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    RPC,
    WebSocket,
    HTTP,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub connection_type: ConnectionType,
    pub is_connected: bool,
    pub is_active: bool,
    pub latency: Option<u64>,
    pub error_message: Option<String>,
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    pub connections: HashMap<String, ConnectionInfo>,
    pub active_connection: Option<String>,
    pub global_latency: Option<u64>,
    pub global_error: Option<String>,
    pub last_updated: std::time::SystemTime,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self {
            connections: HashMap::new(),
            active_connection: None,
            global_latency: None,
            global_error: None,
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Connection signal manager for managing connection state
pub struct ConnectionManager {
    connection_state: AsyncSignalManager<ConnectionState>,
    loading_state: AsyncSignalManager<bool>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connection_state: AsyncSignalManager::new(ConnectionState::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    pub fn add_connection(&mut self, id: String, name: String, url: String, connection_type: ConnectionType) {
        let mut state = self.connection_state.get().value;
        let connection = ConnectionInfo {
            id: id.clone(),
            name,
            url,
            connection_type,
            is_connected: false,
            is_active: false,
            latency: None,
            error_message: None,
            last_updated: std::time::SystemTime::now(),
        };
        state.connections.insert(id, connection);
        state.last_updated = std::time::SystemTime::now();
        self.connection_state.set_success(state);
    }

    pub fn remove_connection(&mut self, id: &str) {
        let mut state = self.connection_state.get().value;
        state.connections.remove(id);
        if state.active_connection.as_ref() == Some(&id.to_string()) {
            state.active_connection = None;
        }
        state.last_updated = std::time::SystemTime::now();
        self.connection_state.set_success(state);
    }

    pub fn set_active_connection(&mut self, id: &str) {
        let mut state = self.connection_state.get().value;
        if state.connections.contains_key(id) {
            // Deactivate all connections
            for connection in state.connections.values_mut() {
                connection.is_active = false;
            }
            // Activate the specified connection
            if let Some(connection) = state.connections.get_mut(id) {
                connection.is_active = true;
            }
            state.active_connection = Some(id.to_string());
            state.last_updated = std::time::SystemTime::now();
            self.connection_state.set_success(state);
        }
    }

    pub fn update_connection_status(&mut self, id: &str, is_connected: bool) {
        let mut state = self.connection_state.get().value;
        if let Some(connection) = state.connections.get_mut(id) {
            connection.is_connected = is_connected;
            connection.last_updated = std::time::SystemTime::now();
            state.last_updated = std::time::SystemTime::now();
            self.connection_state.set_success(state);
        }
    }

    pub fn update_connection_latency(&mut self, latency: u64) {
        let mut state = self.connection_state.get().value;
        state.global_latency = Some(latency);
        state.last_updated = std::time::SystemTime::now();
        self.connection_state.set_success(state);
    }

    pub fn update_connection_error(&mut self, error: String) {
        let mut state = self.connection_state.get().value;
        state.global_error = Some(error);
        state.last_updated = std::time::SystemTime::now();
        self.connection_state.set_success(state);
    }

    pub fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.get().value
    }

    pub fn get_connections(&self) -> HashMap<String, ConnectionInfo> {
        self.connection_state.get().value.connections
    }

    pub fn get_active_connection(&self) -> Option<String> {
        self.connection_state.get().value.active_connection.clone()
    }

    pub fn get_connection(&self, id: &str) -> Option<ConnectionInfo> {
        self.connection_state.get().value.connections.get(id).cloned()
    }

    pub fn subscribe_connections(&self) -> Signal<SignalState<ConnectionState>> {
        self.connection_state.subscribe()
    }

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

// Global connection manager instance
pub static CONNECTION_MANAGER: Signal<ConnectionManager> = Signal::new(ConnectionManager::new);

// Convenience functions for global access
pub fn add_connection(id: String, name: String, url: String, connection_type: ConnectionType) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.add_connection(id, name, url, connection_type);
    CONNECTION_MANAGER.set(manager);
}

pub fn remove_connection(id: &str) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.remove_connection(id);
    CONNECTION_MANAGER.set(manager);
}

pub fn set_active_connection(id: &str) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.set_active_connection(id);
    CONNECTION_MANAGER.set(manager);
}

pub fn update_connection_status(id: &str, is_connected: bool) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.update_connection_status(id, is_connected);
    CONNECTION_MANAGER.set(manager);
}

pub fn update_connection_latency(latency: u64) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.update_connection_latency(latency);
    CONNECTION_MANAGER.set(manager);
}

pub fn update_connection_error(error: String) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.update_connection_error(error);
    CONNECTION_MANAGER.set(manager);
}

pub fn get_connection_state() -> ConnectionState {
    CONNECTION_MANAGER.read().get_connection_state()
}

pub fn get_connections() -> HashMap<String, ConnectionInfo> {
    CONNECTION_MANAGER.read().get_connections()
}

pub fn get_active_connection() -> Option<String> {
    CONNECTION_MANAGER.read().get_active_connection()
}

pub fn get_connection(id: &str) -> Option<ConnectionInfo> {
    CONNECTION_MANAGER.read().get_connection(id)
}

pub fn subscribe_connections() -> Signal<SignalState<ConnectionState>> {
    CONNECTION_MANAGER.read().subscribe_connections()
}

pub fn set_connection_loading(loading: bool) {
    let mut manager = CONNECTION_MANAGER.read().clone();
    manager.set_loading(loading);
    CONNECTION_MANAGER.set(manager);
}

pub fn is_connection_loading() -> bool {
    CONNECTION_MANAGER.read().is_loading()
}

pub fn subscribe_connection_loading() -> Signal<SignalState<bool>> {
    CONNECTION_MANAGER.read().subscribe_loading()
} 