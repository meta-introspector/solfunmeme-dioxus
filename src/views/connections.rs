use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define the Connection struct with cluster support
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Connection {
    name: String,
    url: String,
    cluster: String, // New field to group connections
    // token: String, // Token is now embedded in URL
}

impl Connection {
    // Mask sensitive parts of the URL for display
    pub fn masked_url(&self) -> String {
        if let Some(token_start) = self.url.find("token=") {
            let (before_token, after_token) = self.url.split_at(token_start + 6); // "token=".len() = 6
            if let Some(token_end) = after_token.find('&') {
                let (token_part, rest) = after_token.split_at(token_end);
                let masked_token = if token_part.len() > 8 {
                    format!("{}...{}", &token_part[..4], &token_part[token_part.len()-4..])
                } else {
                    "*".repeat(token_part.len())
                };
                format!("{}{}{}", before_token, masked_token, rest)
            } else {
                // Token is at the end of URL
                let masked_token = if after_token.len() > 8 {
                    format!("{}...{}", &after_token[..4], &after_token[after_token.len()-4..])
                } else {
                    "*".repeat(after_token.len())
                };
                format!("{}{}", before_token, masked_token)
            }
        } else if self.url.contains("access_token=") {
            // Handle access_token parameter
            if let Some(token_start) = self.url.find("access_token=") {
                let (before_token, after_token) = self.url.split_at(token_start + 13); // "access_token=".len() = 13
                if let Some(token_end) = after_token.find('&') {
                    let (token_part, rest) = after_token.split_at(token_end);
                    let masked_token = if token_part.len() > 8 {
                        format!("{}...{}", &token_part[..4], &token_part[token_part.len()-4..])
                    } else {
                        "*".repeat(token_part.len())
                    };
                    format!("{}{}{}", before_token, masked_token, rest)
                } else {
                    let masked_token = if after_token.len() > 8 {
                        format!("{}...{}", &after_token[..4], &after_token[after_token.len()-4..])
                    } else {
                        "*".repeat(after_token.len())
                    };
                    format!("{}{}", before_token, masked_token)
                }
            } else {
                self.url.clone()
            }
        } else if self.url.contains("://") && self.url.matches(':').count() >= 2 {
            // Handle URLs with embedded credentials (user:pass@host)
            if let Some(at_pos) = self.url.find('@') {
                if let Some(scheme_end) = self.url.find("://") {
                    let scheme_part = &self.url[..scheme_end + 3];
                    let after_at = &self.url[at_pos..];
                    format!("{}***{}", scheme_part, after_at)
                } else {
                    self.url.clone()
                }
            } else {
                self.url.clone()
            }
        } else {
            self.url.clone()
        }
    }
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

    pub fn get_by_cluster(&self, cluster: &str) -> Vec<Connection> {
        self.inner
            .read()
            .connections
            .values()
            .filter(|conn| conn.cluster == cluster)
            .cloned()
            .collect()
    }

    pub fn get_clusters(&self) -> Vec<String> {
        let mut clusters: Vec<String> = self.inner
            .read()
            .connections
            .values()
            .map(|conn| conn.cluster.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        clusters.sort();
        clusters
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

    pub fn update(&mut self, old_name: &str, connection: Connection) {
        let mut inner = self.inner.write();
        inner.connections.remove(old_name);
        inner.connections.insert(connection.name.clone(), connection);
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
    let mut new_cluster = use_signal(|| String::new());
    let mut selected_cluster = use_signal(|| String::from("All"));
    let mut editing_connection = use_signal(|| None::<String>);

    // Get available clusters for filter dropdown
    let mut clusters = connections.get_clusters();
    if !clusters.is_empty() {
        clusters.insert(0, "All".to_string());
    }

    // Filter connections based on selected cluster
    let filtered_connections = if selected_cluster() == "All" {
        connections.get_all()
    } else {
        connections.get_by_cluster(&selected_cluster())
    };

    rsx! {
        div { class: "container mx-auto p-6 max-w-4xl",
            h2 { class: "text-2xl font-bold mb-6 text-gray-800 dark:text-white", 
                "Manage Connections & Clusters" 
            }
            
            // Form to add new connection
            div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md mb-6",
                h3 { class: "text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300", 
                    "Add New Connection" 
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1", 
                            "Connection Name" 
                        }
                        input {
                            placeholder: "e.g., Production DB",
                            value: "{new_name}",
                            name: "connection_name",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            oninput: move |event| new_name.set(event.value().clone())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1", 
                            "Cluster" 
                        }
                        input {
                            placeholder: "e.g., Production, Development",
                            value: "{new_cluster}",
                            name: "connection_cluster",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            oninput: move |event| new_cluster.set(event.value().clone())
                        }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1", 
                            "URL (with token)" 
                        }
                        input {
                            placeholder: "https://api.example.com?token=...",
                            value: "{new_url}",
                            name: "connection_url",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            oninput: move |event| new_url.set(event.value().clone())
                        }
                    }
                }
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-colors",
                    onclick: move |_| {
                        let connection = Connection {
                            name: new_name().clone(),
                            url: new_url().clone(),
                            cluster: new_cluster().clone(),
                        };
                        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Adding connection"));
                        if !connection.name.is_empty() && !connection.url.is_empty() && !connection.cluster.is_empty() {
                            connections.add(connection);
                            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("Connection added successfully"));
                            new_name.set(String::new());
                            new_url.set(String::new());
                            new_cluster.set(String::new());
                        }
                    },
                    "Add Connection"
                }
            }

            // Cluster filter
            if !clusters.is_empty() {
                div { class: "mb-6",
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                        "Filter by Cluster:"
                    }
                    select {
                        class: "px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                        value: "{selected_cluster}",
                        onchange: move |event| selected_cluster.set(event.value().clone()),
                        for cluster in clusters {
                            option { value: "{cluster}", "{cluster}" }
                        }
                    }
                }
            }

            // Display connection list
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md",
                div { class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700",
                    h3 { class: "text-lg font-semibold text-gray-800 dark:text-white", 
                        "Connections"
                        if selected_cluster() != "All" {
                            span { class: "text-sm text-gray-500 ml-2", 
                                "({selected_cluster()})" 
                            }
                        }
                    }
                }
                
                if filtered_connections.is_empty() {
                    div { class: "p-6 text-center text-gray-500 dark:text-gray-400",
                        p { "No connections found." }
                    }
                } else {
                    div { class: "divide-y divide-gray-200 dark:divide-gray-700",
                        for conn in filtered_connections {
                            div { class: "p-6 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                                div { class: "flex items-center justify-between",
                                    div { class: "flex-1",
                                        div { class: "flex items-center gap-3 mb-2",
                                            h4 { class: "font-semibold text-gray-800 dark:text-white", 
                                                "{conn.name}" 
                                            }
                                            span { class: "px-2 py-1 bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200 text-xs rounded-full",
                                                "{conn.cluster}"
                                            }
                                        }
                                        p { class: "text-sm text-gray-600 dark:text-gray-400 font-mono break-all",
                                            "{conn.masked_url()}"
                                        }
                                    }
                                    div { class: "flex gap-2",
                                        button {
                                            class: "text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 text-sm font-medium",
                                            onclick: move |_| {
                                                let conn_name = conn.name.clone();
                                                connections.remove(&conn_name)
                                            },
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
}