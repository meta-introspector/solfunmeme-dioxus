use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub name: String,
    pub url: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletState {
    pub is_connected: bool,
    pub public_key: Option<String>,
    pub balance: f64,
    pub network: String,
    pub connection_info: Option<ConnectionInfo>,
    pub last_updated: std::time::SystemTime,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            is_connected: false,
            public_key: None,
            balance: 0.0,
            network: "mainnet".to_string(),
            connection_info: None,
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Wallet signal manager for managing wallet state
pub struct WalletManager {
    wallet_state: AsyncSignalManager<WalletState>,
    loading_state: AsyncSignalManager<bool>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            wallet_state: AsyncSignalManager::new(WalletState::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    pub fn connect(&mut self, public_key: String, connection_info: ConnectionInfo) {
        let mut state = self.wallet_state.get().value;
        state.is_connected = true;
        state.public_key = Some(public_key);
        state.connection_info = Some(connection_info);
        state.last_updated = std::time::SystemTime::now();
        self.wallet_state.set_success(state);
    }

    pub fn disconnect(&mut self) {
        let mut state = self.wallet_state.get().value;
        state.is_connected = false;
        state.public_key = None;
        state.connection_info = None;
        state.last_updated = std::time::SystemTime::now();
        self.wallet_state.set_success(state);
    }

    pub fn update_balance(&mut self, balance: f64) {
        let mut state = self.wallet_state.get().value;
        state.balance = balance;
        state.last_updated = std::time::SystemTime::now();
        self.wallet_state.set_success(state);
    }

    pub fn set_network(&mut self, network: String) {
        let mut state = self.wallet_state.get().value;
        state.network = network;
        state.last_updated = std::time::SystemTime::now();
        self.wallet_state.set_success(state);
    }

    pub fn get_wallet_state(&self) -> WalletState {
        self.wallet_state.get().value
    }

    pub fn is_connected(&self) -> bool {
        self.wallet_state.get().value.is_connected
    }

    pub fn get_balance(&self) -> f64 {
        self.wallet_state.get().value.balance
    }

    pub fn get_public_key(&self) -> Option<String> {
        self.wallet_state.get().value.public_key.clone()
    }

    pub fn subscribe_wallet(&self) -> Signal<SignalState<WalletState>> {
        self.wallet_state.subscribe()
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

// Global wallet manager instance
pub static WALLET_MANAGER: Signal<WalletManager> = Signal::new(WalletManager::new);

// Convenience functions for global access
pub fn connect_wallet(public_key: String, connection_info: ConnectionInfo) {
    let mut manager = WALLET_MANAGER.read().clone();
    manager.connect(public_key, connection_info);
    WALLET_MANAGER.set(manager);
}

pub fn disconnect_wallet() {
    let mut manager = WALLET_MANAGER.read().clone();
    manager.disconnect();
    WALLET_MANAGER.set(manager);
}

pub fn update_wallet_balance(balance: f64) {
    let mut manager = WALLET_MANAGER.read().clone();
    manager.update_balance(balance);
    WALLET_MANAGER.set(manager);
}

pub fn set_wallet_network(network: String) {
    let mut manager = WALLET_MANAGER.read().clone();
    manager.set_network(network);
    WALLET_MANAGER.set(manager);
}

pub fn get_wallet_state() -> WalletState {
    WALLET_MANAGER.read().get_wallet_state()
}

pub fn is_wallet_connected() -> bool {
    WALLET_MANAGER.read().is_connected()
}

pub fn get_wallet_balance() -> f64 {
    WALLET_MANAGER.read().get_balance()
}

pub fn get_wallet_public_key() -> Option<String> {
    WALLET_MANAGER.read().get_public_key()
}

pub fn subscribe_wallet() -> Signal<SignalState<WalletState>> {
    WALLET_MANAGER.read().subscribe_wallet()
}

pub fn set_wallet_loading(loading: bool) {
    let mut manager = WALLET_MANAGER.read().clone();
    manager.set_loading(loading);
    WALLET_MANAGER.set(manager);
}

pub fn is_wallet_loading() -> bool {
    WALLET_MANAGER.read().is_loading()
}

pub fn subscribe_wallet_loading() -> Signal<SignalState<bool>> {
    WALLET_MANAGER.read().subscribe_loading()
} 