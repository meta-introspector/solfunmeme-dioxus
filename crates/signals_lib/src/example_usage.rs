// Example usage of the AOP-style signal management system
// This demonstrates how to register components, menus, and emojis dynamically

use crate::common::{SignalManager, use_signal_manager, SignalState};
use dioxus::prelude::*;

/// Example component showing how to use signal managers
pub fn example_component() -> Element {
    let wallet_manager = use_signal_manager(WalletState {
        is_connected: false,
        public_key: None,
        balance: 0.0,
        network: "mainnet".to_string(),
        connection_info: None,
        last_updated: std::time::SystemTime::now(),
    });

    let loading_manager = use_signal_manager(false);

    rsx! {
        div {
            h2 { "Signal Manager Example" }
            
            // Display wallet state
            div {
                h3 { "Wallet State" }
                p { "Connected: {wallet_manager.get().is_connected}" }
                p { "Balance: {wallet_manager.get().balance}" }
                p { "Network: {wallet_manager.get().network}" }
                
                if wallet_manager.is_loading() {
                    p { "Loading wallet..." }
                }
                
                if let Some(error) = wallet_manager.get_error() {
                    p { style: "color: red;", "Error: {error}" }
                }
            }
            
            // Buttons to interact with wallet
            div {
                button {
                    onclick: move |_| {
                        // In a real component, you'd use use_mut() or similar
                        // For this example, we'll just show the pattern
                    },
                    "Connect Wallet"
                }
                
                button {
                    onclick: move |_| {
                        // In a real component, you'd use use_mut() or similar
                        // For this example, we'll just show the pattern
                    },
                    "Disconnect Wallet"
                }
                
                button {
                    onclick: move |_| {
                        // In a real component, you'd use use_mut() or similar
                        // For this example, we'll just show the pattern
                    },
                    "Simulate Error"
                }
            }
            
            // Loading state example
            div {
                h3 { "Loading State" }
                p { "Loading: {loading_manager.get()}" }
                
                button {
                    onclick: move |_| {
                        // In a real component, you'd use use_mut() or similar
                        // For this example, we'll just show the pattern
                    },
                    "Start Loading"
                }
                
                button {
                    onclick: move |_| {
                        // In a real component, you'd use use_mut() or similar
                        // For this example, we'll just show the pattern
                    },
                    "Stop Loading"
                }
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletState {
    pub is_connected: bool,
    pub public_key: Option<String>,
    pub balance: f64,
    pub network: String,
    pub connection_info: Option<ConnectionInfo>,
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectionInfo {
    pub name: String,
    pub url: String,
    pub is_connected: bool,
}

/// Example of how to create a custom signal manager
pub struct WalletSignalManager {
    wallet: SignalManager<WalletState>,
    loading: SignalManager<bool>,
}

impl WalletSignalManager {
    pub fn new() -> Self {
        Self {
            wallet: use_signal_manager(WalletState {
                is_connected: false,
                public_key: None,
                balance: 0.0,
                network: "mainnet".to_string(),
                connection_info: None,
                last_updated: std::time::SystemTime::now(),
            }),
            loading: use_signal_manager(false),
        }
    }

    pub fn connect_wallet(&mut self, public_key: String, connection_info: ConnectionInfo) {
        self.loading.set_success(true);
        
        // Simulate async operation
        self.wallet.update(|state| {
            state.is_connected = true;
            state.public_key = Some(public_key);
            state.connection_info = Some(connection_info);
        });
        
        self.loading.set_success(false);
    }

    pub fn disconnect_wallet(&mut self) {
        self.wallet.update(|state| {
            state.is_connected = false;
            state.public_key = None;
            state.connection_info = None;
        });
    }

    pub fn update_balance(&mut self, balance: f64) {
        self.wallet.update(|state| {
            state.balance = balance;
        });
    }

    pub fn get_wallet_state(&self) -> WalletState {
        self.wallet.get()
    }

    pub fn subscribe_wallet(&self) -> Signal<SignalState<WalletState>> {
        self.wallet.subscribe()
    }

    pub fn is_loading(&self) -> bool {
        self.loading.get()
    }
} 