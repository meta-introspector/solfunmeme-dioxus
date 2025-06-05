use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the Connection struct
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Connection {
    name: String,
    url: String,
    token: String,
}

// Define the storage entry for persistent state
#[derive(Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    key: String,
    connections: HashMap<String, Connection>, // Keyed by name for uniqueness
}

// Custom hook to manage persistent connections
#[derive(Clone, Copy)]
pub struct UseConnections {
    inner: Signal<StorageEntry>,
}

impl UseConnections {
    pub fn get_all(&self) -> Vec<Connection> {
        self.inner.read().connections.values().cloned().collect()
    }

    pub fn add(&mut self, connection: Connection) {
        let mut inner = self.inner.write();
        inner.connections.insert(connection.name.clone(), connection.clone());
        LocalStorage::set(inner.key.as_str(), &inner.connections).expect("Failed to save to LocalStorage");
    }

    pub fn remove(&mut self, name: &str) {
        let mut inner = self.inner.write();
        inner.connections.remove(name);
        LocalStorage::set(inner.key.as_str(), &inner.connections).expect("Failed to save to LocalStorage");
    }
}

pub fn use_connections(key: impl ToString) -> UseConnections {
    let state = use_signal(move || {
        let key = key.to_string();
        let connections = LocalStorage::get(key.as_str()).ok().unwrap_or_default();
        StorageEntry { key, connections }
    });
    UseConnections { inner: state }
}

// Main Connections component
pub fn Connections() -> Element {
    let mut connections = use_connections("app_connections");
    let mut new_name = use_signal(|| String::new());
    let mut new_url = use_signal(|| String::new());
    let mut new_token = use_signal(|| String::new());

    rsx! {
        div { class: "container",
            h2 { "Manage Connections" }
            // Form to add new connection
            div { class: "form",
                input {
                    placeholder: "Connection Name",
                    value: "{new_name}",
                    oninput: move |event| new_name.set(event.value().clone())
                }
                input {
                    placeholder: "URL",
                    value: "{new_url}",
                    oninput: move |event| new_url.set(event.value().clone())
                }
                input {
                    placeholder: "Token",
                    value: "{new_token}",
                    oninput: move |event| new_token.set(event.value().clone())
                }
                button {
                    onclick: move |_| {
                        let connection = Connection {
                            name: new_name().clone(),
                            url: new_url().clone(),
                            token: new_token().clone(),
                        };
                        if !connection.name.is_empty() && !connection.url.is_empty() && !connection.token.is_empty() {
                            connections.add(connection);
                            new_name.set(String::new());
                            new_url.set(String::new());
                            new_token.set(String::new());
                        }
                    },
                    "Add Connection"
                }
            }
            // Display connection list
            div { class: "connection-list",
                h3 { "Connections" }
                if connections.get_all().is_empty() {
                    p { "No connections added yet." }
                } else {
                    ul {
                        for conn in connections.get_all() {
                            li {
                                div { class: "connection",
                                    span { "{conn.name}: {conn.url} (Token: {conn.token})" }
                                    button {
                                        onclick: move |_| connections.remove(&conn.name),
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// App entry point
//fn main() {
    //launch(Connections);
//}