use dioxus::prelude::*;
use dioxus::core::ScopeState;
use dioxus_use_storage::use_local_storage;


fn build_server_key(name: &str) -> String {
    format!("server_url{}", name)
}

pub fn get_prod_url(cx: &ScopeState, name: &str) -> String {
    let storage = use_local_storage(cx);
    let key = build_server_key(name);
    let current_server = storage.get(&key);
    current_server.unwrap_or("https://api.mainnet-beta.solana.com".to_string())
}

pub fn set_url(cx: &ScopeState, name: &str, value: &str) {
    let storage = use_local_storage(cx);
    let key = build_server_key(name);
    storage.set(&key, value);
}
