use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use thiserror::Error;
//use serde::Deserializer;
//use serde::de::{self, Visitor};
//use std::fmt;

use std::collections::HashMap;
//use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use wallet_adapter::{
    //    web_sys::{self, Window},
    Cluster,
};

//pub(crate) static WINDOW: GlobalSignal<Window> =    Signal::global(|| web_sys::window().expect("Unable to find Window"));

// Define the Connection struct with cluster support
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Connection {
    name: String,
    url: String,
    cluster_name: String, // Reference to cluster by name
}

impl Connection {
    // Mask sensitive parts of the URL for display
    pub fn masked_url(&self) -> String {
        if let Some(token_start) = self.url.find("token=") {
            let (before_token, after_token) = self.url.split_at(token_start + 6); // "token=".len() = 6
            if let Some(token_end) = after_token.find('&') {
                let (token_part, rest) = after_token.split_at(token_end);
                let masked_token = if token_part.len() > 8 {
                    format!(
                        "{}...{}",
                        &token_part[..4],
                        &token_part[token_part.len() - 4..]
                    )
                } else {
                    "*".repeat(token_part.len())
                };
                format!("{}{}{}", before_token, masked_token, rest)
            } else {
                // Token is at the end of URL
                let masked_token = if after_token.len() > 8 {
                    format!(
                        "{}...{}",
                        &after_token[..4],
                        &after_token[after_token.len() - 4..]
                    )
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
                        format!(
                            "{}...{}",
                            &token_part[..4],
                            &token_part[token_part.len() - 4..]
                        )
                    } else {
                        "*".repeat(token_part.len())
                    };
                    format!("{}{}{}", before_token, masked_token, rest)
                } else {
                    let masked_token = if after_token.len() > 8 {
                        format!(
                            "{}...{}",
                            &after_token[..4],
                            &after_token[after_token.len() - 4..]
                        )
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

//struct MyCluster(Cluster); // Wrapper type

/// Used as a helper struct to contain all the features supported by a wallet
/// as defined by the wallet standard

/// Solana Clusters
#[derive(
    Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
pub enum MyCluster {
    /// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
    MainNet,
    /// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
    #[default]
    DevNet,
    /// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
    TestNet,
    /// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
    LocalNet,
}

/// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
pub const MAINNET_IDENTIFIER: &str = "solana:mainnet";
/// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
pub const DEVNET_IDENTIFIER: &str = "solana:devnet";
/// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
pub const TESTNET_IDENTIFIER: &str = "solana:testnet";
/// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
pub const LOCALNET_IDENTIFIER: &str = "solana:localnet";

/// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
pub const MAINNET: &str = "mainnet";
/// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
pub const DEVNET: &str = "devnet";
/// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
pub const TESTNET: &str = "testnet";
/// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
pub const LOCALNET: &str = "localnet";

/// Solana Mainnet cluster
pub const MAINNET_ENDPOINT: &str = "https://api.mainnet-beta.solana.com";
/// Solana Devnet cluster
pub const DEVNET_ENDPOINT: &str = "https://api.devnet.solana.com";
/// Solana Testnet cluster
pub const TESTNET_ENDPOINT: &str = "https://api.testnet.solana.com";
/// Solana Localnet cluster
pub const LOCALNET_ENDPOINT: &str = "https://solana.solfunmeme.com/validator/";
impl MyCluster {
    /// A Solana endpoint URI
    pub fn endpoint(&self) -> &str {
        match self {
            MyCluster::MainNet => MAINNET_ENDPOINT,
            MyCluster::DevNet => DEVNET_ENDPOINT,
            MyCluster::TestNet => TESTNET_ENDPOINT,
            MyCluster::LocalNet => LOCALNET_ENDPOINT,
        }
    }

    /// A Solana cluster identifier
    // pub fn chain(&self) -> &str {
    //     match self {
    //         MyCluster::MainNet => MAINNET_IDENTIFIER,
    //         MyCluster::DevNet => DEVNET_IDENTIFIER,
    //         MyCluster::TestNet => TESTNET_IDENTIFIER,
    //         MyCluster::LocalNet => LOCALNET_IDENTIFIER,
    //     }
    // }

    /// A Solana cluster identifier as a &str
    pub fn display(&self) -> &str {
        match self {
            MyCluster::MainNet => MAINNET,
            MyCluster::DevNet => DEVNET,
            MyCluster::TestNet => TESTNET,
            MyCluster::LocalNet => LOCALNET,
        }
    }
}

impl core::fmt::Display for MyCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Error handling enum
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Error)]
pub enum MyWalletError {
    /// Unable to send the a [WalletEvent] via the [crate::WalletEventSender]
    //    #[error("Unable to send the a `WalletEvent` variant via the WalletEventSender channel")]
    //    ChannelError,
    #[error("Unsupported")]
    UnsupportedChain(String),
}

impl TryFrom<&str> for MyCluster {
    type Error = MyWalletError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cluster = match value {
            MAINNET_IDENTIFIER => Self::MainNet,
            DEVNET_IDENTIFIER => Self::DevNet,
            TESTNET_IDENTIFIER => Self::TestNet,
            LOCALNET_IDENTIFIER => Self::LocalNet,
            MAINNET_ENDPOINT => Self::MainNet,
            DEVNET_ENDPOINT => Self::DevNet,
            TESTNET_ENDPOINT => Self::TestNet,
            LOCALNET_ENDPOINT => Self::LocalNet,
            MAINNET => Self::MainNet,
            DEVNET => Self::DevNet,
            TESTNET => Self::TestNet,
            LOCALNET => Self::LocalNet,
            _ => return Err(MyWalletError::UnsupportedChain(value.to_string())),
        };

        Ok(cluster)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct AdapterCluster {
    name: String,
    cluster: MyCluster,
    endpoint: String,
}

impl AdapterCluster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn add_cluster(mut self, cluster: MyCluster) -> Self {
        self.cluster = cluster;
        self
    }

    pub fn add_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn cluster(&self) -> MyCluster {
        self.cluster
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    // pub fn identifier(&self) -> String {
    //     self.to_string()
    // }

    // pub fn query_string(&self) -> String {
    //     if self.name.as_bytes() == self.cluster.to_string().as_bytes()
    //         && self.cluster != MyCluster::LocalNet
    //     {
    //         String::new() + "?cluster=" + self.cluster.to_string().as_str()
    //     } else {
    //         String::new()
    //             + "?cluster=custom&customUrl="
    //             + utf8_percent_encode(self.endpoint.as_str(), NON_ALPHANUMERIC)
    //                 .to_string()
    //                 .as_str()
    //     }
    // }

    pub fn devnet() -> Self {
        AdapterCluster {
            name: "devnet".to_string(),
            cluster: MyCluster::DevNet,
            endpoint: MyCluster::DevNet.endpoint().to_string(),
        }
    }

    pub fn mainnet() -> Self {
        AdapterCluster {
            name: "mainnet".to_string(),
            cluster: MyCluster::MainNet,
            endpoint: MyCluster::MainNet.endpoint().to_string(),
        }
    }

    pub fn testnet() -> Self {
        AdapterCluster {
            name: "testnet".to_string(),
            cluster: MyCluster::TestNet,
            endpoint: MyCluster::TestNet.endpoint().to_string(),
        }
    }

    pub fn localnet() -> Self {
        AdapterCluster {
            name: "localnet".to_string(),
            cluster: MyCluster::LocalNet,
            endpoint: MyCluster::LocalNet.endpoint().to_string(),
        }
    }

    // Mask sensitive parts of the endpoint for display
    pub fn masked_endpoint(&self) -> String {
        Connection {
            name: self.name.clone(),
            url: self.endpoint.clone(),
            cluster_name: self.name.clone(),
        }
        .masked_url()
    }
}

impl Default for AdapterCluster {
    fn default() -> Self {
        Self::devnet()
    }
}

impl std::fmt::Display for AdapterCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cluster.display())
    }
}

// Define the storage entry for persistent state
#[derive(Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    key: String,
    connections: HashMap<String, Connection>, // Keyed by name for uniqueness
    clusters: Vec<AdapterCluster>,            // Store clusters separately
}

// Custom hook to manage persistent connections and clusters
#[derive(Clone, Copy)]
pub struct UseConnections {
    inner: Signal<StorageEntry>,
}

impl UseConnections {
    pub fn get_all_connections(&self) -> Vec<Connection> {
        self.inner.read().connections.values().cloned().collect()
    }

    pub fn get_connections_by_cluster(&self, cluster_name: &str) -> Vec<Connection> {
        self.inner
            .read()
            .connections
            .values()
            .filter(|conn| conn.cluster_name == cluster_name)
            .cloned()
            .collect()
    }

    pub fn get_all_clusters(&self) -> Vec<AdapterCluster> {
        self.inner.read().clusters.clone()
    }

    pub fn get_cluster_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .inner
            .read()
            .clusters
            .iter()
            .map(|cluster| cluster.name().to_string())
            .collect();
        names.sort();
        names
    }

    pub fn add_connection(&mut self, connection: Connection) {
        let mut inner = self.inner.write();
        inner
            .connections
            .insert(connection.name.clone(), connection.clone());
        LocalStorage::set(&format!("{}_connections", inner.key), &inner.connections)
            .expect("Failed to save connections to LocalStorage");
    }

    // pub fn remove_connection(&mut self, name: &str) {
    //     let mut inner = self.inner.write();
    //     inner.connections.remove(name);
    //     LocalStorage::set(&format!("{}_connections", inner.key), &inner.connections)
    //         .expect("Failed to save connections to LocalStorage");
    // }

    pub fn add_cluster(&mut self, cluster: AdapterCluster) -> Result<(), String> {
        let mut inner = self.inner.write();

        // Check if cluster already exists
        let cluster_exists = inner.clusters.iter().any(|existing_cluster| {
            existing_cluster.name() == cluster.name()
                || existing_cluster.endpoint() == cluster.endpoint()
        });

        if cluster_exists {
            return Err("Cluster exists, make sure endpoint or name are not the same".to_string());
        }

        inner.clusters.push(cluster);
        LocalStorage::set(&format!("{}_clusters", inner.key), &inner.clusters)
            .expect("Failed to save clusters to LocalStorage");
        Ok(())
    }

    pub fn remove_cluster(&mut self, cluster_name: &str) -> Option<AdapterCluster> {
        let mut inner = self.inner.write();

        // Find and remove cluster
        let position = inner
            .clusters
            .iter()
            .position(|cluster| cluster.name() == cluster_name)?;

        let removed_cluster = inner.clusters.remove(position);

        // Also remove all connections that reference this cluster
        inner
            .connections
            .retain(|_, conn| conn.cluster_name != cluster_name);

        // Save both updated collections
        LocalStorage::set(&format!("{}_clusters", inner.key), &inner.clusters)
            .expect("Failed to save clusters to LocalStorage");
        LocalStorage::set(&format!("{}_connections", inner.key), &inner.connections)
            .expect("Failed to save connections to LocalStorage");

        Some(removed_cluster)
    }

    // pub fn get_cluster(&self, name: &str) -> Option<AdapterCluster> {
    //     self.inner
    //         .read()
    //         .clusters
    //         .iter()
    //         .find(|cluster| cluster.name() == name)
    //         .cloned()
    // }
}

pub fn use_connections(key: impl ToString) -> UseConnections {
    let state = use_signal(move || {
        let key = key.to_string();

        // Load connections
        let connections: HashMap<String, Connection> =
            LocalStorage::get(&format!("{}_connections", key))
                .ok()
                .unwrap_or_default();

        // Load clusters, with default clusters if none exist
        let mut clusters: Vec<AdapterCluster> = LocalStorage::get(&format!("{}_clusters", key))
            .ok()
            .unwrap_or_else(|| {
                vec![
                    AdapterCluster::devnet(),
                    AdapterCluster::testnet(),
                    AdapterCluster::mainnet(),
                    AdapterCluster::localnet(),
                ]
            });

        // Ensure default clusters exist
        if clusters.is_empty() {
            clusters = vec![
                AdapterCluster::devnet(),
                AdapterCluster::testnet(),
                AdapterCluster::mainnet(),
                AdapterCluster::localnet(),
            ];
        }

        StorageEntry {
            key,
            connections,
            clusters,
        }
    });
    UseConnections { inner: state }
}

pub fn connection_management_section(
    cluster_names: Vec<String>,
    mut new_connection_name: Signal<String>,
    mut new_connection_url: Signal<String>,
    mut selected_cluster_for_connection: Signal<String>,
    //filtered_connections: Vec<Connection>,
    //connections: &UseConnections,
    //selected_cluster_filter: Signal<String>
) -> Element {
    let mut connections = use_connections("app_data");
    rsx! {
                // Connection Management Section
            div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md mb-6",
                h3 { class: "text-lg font-semibold mb-4 text-gray-700 dark:text-gray-300",
                    "Add New Connection"
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-4",
                    div {
                        label {
			    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
			    for: "connection-name",
                            "Connection Name"
                        }

                        input {
                            placeholder: "e.g., My App Connection",
                            value: "{new_connection_name}",
			    id:  "connection-name",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            oninput: move |event| new_connection_name.set(event.value().clone())
                        }
                    }
                    div {
                        label {
			    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
			    for: "select_cluster",
                            "Cluster"
                        }
                        select {
                id: "select_cluster",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            value: "{selected_cluster_for_connection}",
                            onchange: move |event| selected_cluster_for_connection.set(event.value().clone()),
                            option { value: "", "Select a cluster..." }
                            for cluster_name in cluster_names.clone() {
                                option { value: "{cluster_name}", "{cluster_name}" }
                            }
                        }
                    }
                    div {
                        label {
			    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
			    for : "api-url",
                            "URL (with token)"
                        }
                        input {
                            placeholder: "https://api.example.com?token=...",
                            value: "{new_connection_url}",
			    id: "api-url",
                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            oninput: move |event| new_connection_url.set(event.value().clone())
                        }
                    }
                }
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-colors",
                    onclick: move |_| {
                        let connection = Connection {
                            name: new_connection_name().clone(),
                            url: new_connection_url().clone(),
                            cluster_name: selected_cluster_for_connection().clone(),
                        };
                        if !connection.name.is_empty() && !connection.url.is_empty() && !connection.cluster_name.is_empty() {
                            connections.add_connection(connection);
                            new_connection_name.set(String::new());
                            new_connection_url.set(String::new());
                            selected_cluster_for_connection.set(String::new());
                        }
                    },
                    "Add Connection"
                }
            }
    }
}

fn list_connections(
    filtered_connections: &Vec<Connection>, //, connections: &UseConnections
) -> Element {
    //        let mut connections = use_connections("app_data");
    rsx! {
    div { class: "divide-y divide-gray-200 dark:divide-gray-700",
          for conn in filtered_connections {
            { web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("conn: {}", conn.name))); }
              div { class: "p-6 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                    div { class: "flex items-center justify-between",
                                div { class: "flex-1",
                                    div { class: "flex items-center gap-3 mb-2",
                                        h4 { class: "font-semibold text-gray-800 dark:text-white",
                                            "{conn.name}"
                                        }
                                        span { class: "px-2 py-1 bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200 text-xs rounded-full",
                                            "{conn.cluster_name}"
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
                                            //let conn_name = conn.name.clone();
                                            //connections.remove_connection(&conn_name)
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

fn connection_filter(
    filter_options: Vec<String>,
    //filtered_connections: Vec<Connection>,
    //connections: &UseConnections,
    mut selected_cluster_filter: Signal<String>,
) -> Element {
    rsx! {
            // Connection Filter and List
            if !filter_options.is_empty() {
                div {
		    class: "mb-6",
                    label {
			class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
			for: "filter-cluster",
                        "Filter connections by cluster:"
                    }
                    select {
                name: 	"filter-cluster",
                        class: "px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                        value: "{selected_cluster_filter}",
                        onchange: move |event| selected_cluster_filter.set(event.value().clone()),
                        for cluster in filter_options {
                            option { value: "{cluster}", "{cluster}" }
                        }
                    }
                }
            }
    }
}

pub fn connection_list(
    filtered_connections: &Vec<Connection>,
    //connections: &UseConnections,
    selected_cluster_filter: Signal<String>,
) -> Element {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("get: {}", selected_cluster_filter())));
    rsx! {
         // Display connection list
        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-md",
            div { class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700",
                h3 { class: "text-lg font-semibold text-gray-800 dark:text-white",
                    "Connections"
                    if selected_cluster_filter() != "All" {
                        span { class: "text-sm text-gray-500 ml-2",
                            "({selected_cluster_filter()})"
                        }
                    }
                }
            }

            if filtered_connections.is_empty() {
                div { class: "p-6 text-center text-gray-500 dark:text-gray-400",
                    p { "No connections found." }
                }
            } else {
               //list_connections(filtered_connections, &connections)
                {
		    let res = list_connections(filtered_connections);
//   	            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("res: {:#?}", res)));
		    match res {
			Ok(rst) => {
			    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("rst1: {:#?}", rst)));
			    rst
			},
			Err(_err) => rsx! { div { "error"} }?,
        }
        }
            }
        }
    }
}

pub fn cluster_management_section(
    mut new_cluster_name: Signal<String>,
    mut new_cluster_type: Signal<String>,
    mut new_cluster_endpoint: Signal<String>,
    mut show_cluster_form: Signal<bool>,
    mut connections: UseConnections,
) -> Element {
    rsx! {
                   // Cluster Management Section
            div { class: "bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md mb-6",
                div { class: "flex items-center justify-between mb-4",
                    h3 { class: "text-lg font-semibold text-gray-700 dark:text-gray-300",
                        "Cluster Management"
                    }
                    button {
                        class: "bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-md transition-colors text-sm",
                        onclick: move |_| show_cluster_form.set(!show_cluster_form()),
                        if show_cluster_form() { "Cancel" } else { "Add Cluster" }
                    }
                }

                // Add Cluster Form (conditional)
                if show_cluster_form() {
                    div { class: "border-t pt-4 mb-4",
                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-4",
                            div {
                                label {
				    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
				    for: "cluster-name",
                                    "Cluster Name"
                                }
                                input {
                                    placeholder: "e.g., my-custom-cluster",
                                    value: "{new_cluster_name}",
                    name: "cluster-name",
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                                    oninput: move |event| new_cluster_name.set(event.value().clone())
                                }
                            }
                            div {
                                label {
				    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
				    for: "cluster-type",
                                    "Cluster Type"
                                }
                                select {
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                                    value: "{new_cluster_type}",
                        name: "cluster-type",
                                    onchange: move |event| {
                                        new_cluster_type.set(event.value().clone());
                                        // Auto-fill endpoint based on type
                                        match event.value().as_str() {
                                            "devnet" => new_cluster_endpoint.set(Cluster::DevNet.endpoint().to_string()),
                                            "testnet" => new_cluster_endpoint.set(Cluster::TestNet.endpoint().to_string()),
                                            "mainnet" => new_cluster_endpoint.set(Cluster::MainNet.endpoint().to_string()),
                                            "localnet" => new_cluster_endpoint.set(Cluster::LocalNet.endpoint().to_string()),
                                            _ => {}
                                        }
                                    },
                                    option { value: "custom", "Custom" }
                                    option { value: "devnet", "DevNet" }
                                    option { value: "testnet", "TestNet" }
                                    option { value: "mainnet", "MainNet" }
                                    option { value: "localnet", "LocalNet" }
                                }
                            }
                            div {
                                label {
				    for: "endpoint-url",
				    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                    "Endpoint URL"
                                }
                                input {
                                    placeholder: "https://api.devnet.solana.com",
                                    value: "{new_cluster_endpoint}",
                    name: "endpoint-url",
                                    class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                                    oninput: move |event| new_cluster_endpoint.set(event.value().clone())
                                }
                            }
                        }
                        button {
                            class: "bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-md transition-colors",
                            onclick: move |_| {
                                if !new_cluster_name().is_empty() && !new_cluster_endpoint().is_empty() {
                                    let cluster_type = match new_cluster_type().as_str() {
                                        "devnet" => MyCluster::DevNet,
                                        "testnet" => MyCluster::TestNet,
                                        "mainnet" => MyCluster::MainNet,
                                        "localnet" => MyCluster::LocalNet,
                                        _ => MyCluster::DevNet, // Default for custom
                                    };

                                    let cluster = AdapterCluster::new()
                                        .add_name(&new_cluster_name())
                                        .add_cluster(cluster_type)
                                        .add_endpoint(&new_cluster_endpoint());

                                    match connections.add_cluster(cluster) {
                                        Ok(_) => {
                                            new_cluster_name.set(String::new());
                                            new_cluster_endpoint.set(String::new());
                                            new_cluster_type.set(String::from("custom"));
                                            show_cluster_form.set(false);
                                        }
                                        Err(e) => {
                                            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Error: {}", e)));
                                        }
                                    }
                                }
                            },
                            "Add Cluster"
                        }
                    }
                }

                // Display clusters
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for cluster in connections.get_all_clusters() {
                        div { class: "border border-gray-200 dark:border-gray-600 rounded-lg p-4",
                            div { class: "flex items-center justify-between mb-2",
                                h4 { class: "font-semibold text-gray-800 dark:text-white",
                                    "{cluster.name()}"
                                }
                                button {
                                    class: "text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 text-sm",
                                    onclick: move |_| {
                                        let cluster_name = cluster.name().to_string();
                                        connections.remove_cluster(&cluster_name);
                                    },
                                    "Delete"
                                }
                            }
                            p { class: "text-sm text-gray-600 dark:text-gray-400 mb-1",
                                "Type: {cluster.cluster().display()}"
                            }
                            p { class: "text-xs text-gray-500 dark:text-gray-500 font-mono break-all",
                                "{cluster.masked_endpoint()}"
                            }
                        }
                    }
                }
            }


    }
}

// Main Connections component
pub fn Connections() -> Element {
    let connections = use_connections("app_data");
    let new_connection_name = use_signal(|| String::new());
    let new_connection_url = use_signal(|| String::new());
    let selected_cluster_for_connection = use_signal(|| String::new());
    let selected_cluster_filter: Signal<String> = use_signal(|| String::from("All"));

    // Cluster management state
    let new_cluster_name = use_signal(|| String::new());
    let new_cluster_endpoint = use_signal(|| String::new());
    let new_cluster_type = use_signal(|| String::from("custom"));
    let show_cluster_form = use_signal(|| false);

    // Get available clusters for dropdowns
    let cluster_names = connections.get_cluster_names();
    let mut filter_options = cluster_names.clone();
    filter_options.insert(0, "All".to_string());

    // Filter connections based on selected cluster
    let filtered_connections = if selected_cluster_filter() == "All" {
        connections.get_all_connections()
    } else {
        connections.get_connections_by_cluster(&selected_cluster_filter())
    };

    rsx! {
        div { class: "container mx-auto p-6 max-w-6xl",
            h2 { class: "text-3xl font-bold mb-8 text-gray-800 dark:text-white",
                "Manage Connections & Clusters"
            }
          { cluster_management_section(new_cluster_name,
                       new_cluster_type,
                       new_cluster_endpoint,
                        show_cluster_form,
                       connections )
                    }





           {  connection_management_section(
               cluster_names,
               new_connection_name,
               new_connection_url,
               selected_cluster_for_connection,
               //filtered_connections,
               //& connections,
               //selected_cluster_filter
           ) }


	  { connection_filter(   filter_options,       selected_cluster_filter ); }
	  { connection_list(&filtered_connections,                   selected_cluster_filter) }
	      
          // { let res1 = connection_filter(              filter_options,              selected_cluster_filter);
          //   match res1 {
	  // 	//Ok(rst) => rst,
	  // 	Ok(rst) => {
	  // 	    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("rst4: {:#?}", rst)));
	  // 	    rst
	  // 	},
		
	  // 	//              Err(_err) => "err",
	  // 	Err(_err) => rsx! { div { "error1"} }?,
          //   };
	    
	  //   let res = connection_list(&filtered_connections,
	  // 			      //&connections,
	  // 			      selected_cluster_filter);
	    
          //   match res {
	  // 	//Ok(rst) => rst,
	  // 	Ok(rst) => {
	  // 	    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("rst3: {:#?}", rst)));
	  // 	    rst
	  // 	},
		
	  // 	Err(_err) => rsx! { div { "error2"} }?,
	  // 	//Err(_err) => "err",
          //   };
          // }
	      
	      
	      //      connection_list {filtered_connections,selected_cluster_filter}
	      
	      
	      
        }
    }
}
