use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAccount {
    pub mint: String,
    pub balance: f64,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub address: String,
    pub name: String,
    pub balance: f64,
    pub token_accounts: Vec<TokenAccount>,
    pub is_active: bool,
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub accounts: HashMap<String, AccountInfo>,
    pub active_account: Option<String>,
    pub last_updated: std::time::SystemTime,
}

impl Default for AccountState {
    fn default() -> Self {
        Self {
            accounts: HashMap::new(),
            active_account: None,
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Account signal manager for managing account state
pub struct AccountManager {
    account_state: AsyncSignalManager<AccountState>,
    loading_state: AsyncSignalManager<bool>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            account_state: AsyncSignalManager::new(AccountState::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    pub fn add_account(&mut self, address: String, name: String) {
        let mut state = self.account_state.get().value;
        let account = AccountInfo {
            address: address.clone(),
            name,
            balance: 0.0,
            token_accounts: Vec::new(),
            is_active: false,
            last_updated: std::time::SystemTime::now(),
        };
        state.accounts.insert(address, account);
        state.last_updated = std::time::SystemTime::now();
        self.account_state.set_success(state);
    }

    pub fn remove_account(&mut self, address: &str) {
        let mut state = self.account_state.get().value;
        state.accounts.remove(address);
        if state.active_account.as_ref() == Some(&address.to_string()) {
            state.active_account = None;
        }
        state.last_updated = std::time::SystemTime::now();
        self.account_state.set_success(state);
    }

    pub fn set_active_account(&mut self, address: &str) {
        let mut state = self.account_state.get().value;
        if state.accounts.contains_key(address) {
            // Deactivate all accounts
            for account in state.accounts.values_mut() {
                account.is_active = false;
            }
            // Activate the specified account
            if let Some(account) = state.accounts.get_mut(address) {
                account.is_active = true;
            }
            state.active_account = Some(address.to_string());
            state.last_updated = std::time::SystemTime::now();
            self.account_state.set_success(state);
        }
    }

    pub fn update_account_balance(&mut self, address: &str, balance: f64) {
        let mut state = self.account_state.get().value;
        if let Some(account) = state.accounts.get_mut(address) {
            account.balance = balance;
            account.last_updated = std::time::SystemTime::now();
            state.last_updated = std::time::SystemTime::now();
            self.account_state.set_success(state);
        }
    }

    pub fn update_token_accounts(&mut self, address: &str, token_accounts: Vec<TokenAccount>) {
        let mut state = self.account_state.get().value;
        if let Some(account) = state.accounts.get_mut(address) {
            account.token_accounts = token_accounts;
            account.last_updated = std::time::SystemTime::now();
            state.last_updated = std::time::SystemTime::now();
            self.account_state.set_success(state);
        }
    }

    pub fn get_account_state(&self) -> AccountState {
        self.account_state.get().value
    }

    pub fn get_accounts(&self) -> HashMap<String, AccountInfo> {
        self.account_state.get().value.accounts
    }

    pub fn get_active_account(&self) -> Option<String> {
        self.account_state.get().value.active_account.clone()
    }

    pub fn get_account(&self, address: &str) -> Option<AccountInfo> {
        self.account_state.get().value.accounts.get(address).cloned()
    }

    pub fn subscribe_accounts(&self) -> Signal<SignalState<AccountState>> {
        self.account_state.subscribe()
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

// Global account manager instance
pub static ACCOUNT_MANAGER: Signal<AccountManager> = Signal::new(AccountManager::new);

// Convenience functions for global access
pub fn add_account(address: String, name: String) {
    let mut manager = ACCOUNT_MANAGER.read().clone();
    manager.add_account(address, name);
    ACCOUNT_MANAGER.set(manager);
}

pub fn remove_account(address: &str) {
    let mut manager = ACCOUNT_MANAGER.read().clone();
    manager.remove_account(address);
    ACCOUNT_MANAGER.set(manager);
}

pub fn set_active_account(address: &str) {
    let mut manager = ACCOUNT_MANAGER.read().clone();
    manager.set_active_account(address);
    ACCOUNT_MANAGER.set(manager);
}

pub fn update_account_balance(address: &str, balance: f64) {
    let mut manager = ACCOUNT_MANAGER.read().clone();
    manager.update_account_balance(address, balance);
    ACCOUNT_MANAGER.set(manager);
}

pub fn update_token_accounts(address: &str, token_accounts: Vec<TokenAccount>) {
    let mut manager = ACCOUNT_MANAGER.read().clone();
    manager.update_token_accounts(address, token_accounts);
    ACCOUNT_MANAGER.set(manager);
}

pub fn get_account_state() -> AccountState {
    ACCOUNT_MANAGER.read().get_account_state()
}

pub fn get_accounts() -> HashMap<String, AccountInfo> {
    ACCOUNT_MANAGER.read().get_accounts()
}

pub fn get_active_account() -> Option<String> {
    ACCOUNT_MANAGER.read().get_active_account()
}

pub fn get_account(address: &str) -> Option<AccountInfo> {
    ACCOUNT_MANAGER.read().get_account(address)
}

pub fn subscribe_accounts() -> Signal<SignalState<AccountState>> {
    ACCOUNT_MANAGER.read().subscribe_accounts()
}

pub fn set_account_loading(loading: bool) {
    let mut manager = ACCOUNT_MANAGER.read().clone();
    manager.set_loading(loading);
    ACCOUNT_MANAGER.set(manager);
}

pub fn is_account_loading() -> bool {
    ACCOUNT_MANAGER.read().is_loading()
}

pub fn subscribe_account_loading() -> Signal<SignalState<bool>> {
    ACCOUNT_MANAGER.read().subscribe_loading()
} 