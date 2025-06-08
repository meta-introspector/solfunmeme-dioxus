use dioxus::prelude::*;
use crate::{
    model::{adaptercluster::AdapterCluster, storage::GLOBAL_MESSAGE, storage_entry::StorageEntry, Connection, MyCluster},
    utils::{get_cluster_svg, trunk_cluster_name},
    //AdapterCluster,
     BinSvg, CheckSvg, CloseSvg, ClusterName, ClustersSvg, LinkSvg, NotificationInfo,
};
use gloo_storage::{LocalStorage, Storage};
use url::Url;
use std::collections::HashMap;

// Custom hook to manage persistent connections and clusters
#[derive(Clone, Copy, PartialEq)]
pub struct UseConnections {
    inner: Signal<StorageEntry>,
    active_cluster: Signal<String>,
}

impl UseConnections {
    pub fn get_all_clusters(&self) -> Vec<AdapterCluster> {
        self.inner.read().clusters.clone()
    }

    pub fn get_cluster(&self, name: &str) -> Option<AdapterCluster> {
        self.inner.read().clusters.iter().find(|c| c.name() == name).cloned()
    }

    pub fn add_cluster(&mut self, cluster: AdapterCluster) -> Result<(), String> {
        let mut inner = self.inner.write();
        let cluster_exists = inner.clusters.iter().any(|existing_cluster| {
            existing_cluster.name() == cluster.name() || existing_cluster.endpoint() == cluster.endpoint()
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
        let position = inner.clusters.iter().position(|cluster| cluster.name() == cluster_name)?;
        let removed_cluster = inner.clusters.remove(position);
        inner.connections.retain(|_, conn| conn.cluster_name != cluster_name);
        LocalStorage::set(&format!("{}_clusters", inner.key), &inner.clusters)
            .expect("Failed to save clusters to LocalStorage");
        LocalStorage::set(&format!("{}_connections", inner.key), &inner.connections)
            .expect("Failed to save connections to LocalStorage");
        if *self.active_cluster.read() == cluster_name {
            self.active_cluster.set(
                inner.clusters.first().map(|c| c.name().to_string()).unwrap_or_default()
            );
            LocalStorage::set(&format!("{}_active_cluster", inner.key), &*self.active_cluster.read())
                .expect("Failed to save active cluster");
        }
        Some(removed_cluster)
    }

    pub fn set_active_cluster(&mut self, cluster_name: String) {
        self.active_cluster.set(cluster_name.clone());
        LocalStorage::set(&format!("{}_active_cluster", self.inner.read().key), &cluster_name)
            .expect("Failed to save active cluster");
    }

    pub fn active_cluster(&self) -> String {
        self.active_cluster.read().clone()
    }
}

pub fn use_connections(key: impl ToString) -> UseConnections {
    let key = key.to_string();
    let key_for_state = key.clone();
    let key_for_active = key.clone();
    let state = use_signal(move || {
        let connections: HashMap<String, Connection> = LocalStorage::get(&format!("{}_connections", &key_for_state))
            .ok()
            .unwrap_or_default();
        let mut clusters: Vec<AdapterCluster> = LocalStorage::get(&format!("{}_clusters", &key_for_state))
            .ok()
            .unwrap_or_else(|| {
                vec![
                    AdapterCluster::devnet(),
                    AdapterCluster::testnet(),
                    AdapterCluster::mainnet(),
                    AdapterCluster::localnet(),
                ]
            });
        if clusters.is_empty() {
            clusters = vec![
                AdapterCluster::devnet(),
                AdapterCluster::testnet(),
                AdapterCluster::mainnet(),
                AdapterCluster::localnet(),
            ];
        }
        StorageEntry { key: key_for_state.clone(), connections, clusters }
    });
    let active_cluster = use_signal(move || {
        LocalStorage::get(&format!("{}_active_cluster", &key_for_active))
            .ok()
            .unwrap_or_else(|| state.read().clusters.first().map(|c| c.name().to_string()).unwrap_or_default())
    });
    UseConnections { inner: state, active_cluster }
}

#[component]
pub fn Clusters() -> Element {
    let mut connections = use_connections("solana_wallet");
    let mut show_add_cluster_modal = use_signal(|| false);

    rsx! {
        div {
            class: "flex w-full flex-col justify-start p-10 items-center",
            div {
                class: "flex flex-col w-full items-center justify-center text-4xl",
                span { class: "flex w-[100px]", {ClustersSvg()} }
                "Clusters"
                div { class: "text-xl", "Manage your Solana endpoints" }
                button {
                    onclick: move |_| show_add_cluster_modal.set(true),
                    class: "bg-true-blue text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue",
                    "ADD CLUSTER"
                }
                div {
                    class: "flex flex-wrap w-full items-stretch justify-center gap-4 mt-20",
                    ClusterInfo { connections: connections }
                }
            }
        }
        AddClusterModal { show_add_cluster_modal, connections }
    }
}

#[component(partial_eq = false)]
pub fn ClusterInfo(connections: UseConnections) -> Element {
    let active_cluster_name = connections.active_cluster();
    let clusters = connections.get_all_clusters();

    rsx! {
        {clusters.iter().map(|adapter_cluster| {
            let is_active = adapter_cluster.name() == active_cluster_name;
            rsx! {
                div {
                    class: "flex flex-col text-xl p-5 w-[250px] bg-true-blue rounded-xl",
                    div {
                        class: "flex w-full",
                        span { class: "w-[25px] mr-2", {get_cluster_svg(adapter_cluster.cluster())()} }
                        {trunk_cluster_name(adapter_cluster.name())}
                    }
                    div {
                        class: "flex flex-col w-full",
                        div {
                            class: "flex w-full items-start flex-col mt-2.5 mb-5",
                            div {
                                class: "bg-blue-100 text-blue-800 text-sm font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800",
                                {adapter_cluster.cluster().chain()}
                            }
                            div { class: "text-sm mt-2", {adapter_cluster.endpoint()} }
                        }
                        div {
                            class: "flex w-full items-center justify-between",
                            if !is_active {
                                div {
                                    class: "text-3xl font-bold text-gray-900 dark:text-white",
                                    {Switch(adapter_cluster.name().to_string(), connections)}
                                }
                                div {
                                    class: "hover:bg-blue-800 rounded-xl dark:hover:bg-blue-700",
                                    {Delete(adapter_cluster.name().to_string(), connections)}
                                }
                            } else {
                                span { class: "w-5", {CheckSvg()} }
                            }
                        }
                    }
                }
            }
        })}
    }
}

fn Switch(cluster_name: String, mut connections: UseConnections) -> Element {
    let cluster_name_for_input = cluster_name.clone();
    rsx! {
        label {
            onclick: move |_| {
                if let Some(active_cluster) = connections.get_cluster(&cluster_name) {
                    connections.set_active_cluster(cluster_name.clone());
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("{} cluster now active!", cluster_name)));
                } else {
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("Could not find `{}` cluster!", cluster_name)));
                }
            },
            title: "Switch",
            class: "inline-flex items-center cursor-pointer",
            input {
                name: format!("sr-only-peer-{}", cluster_name_for_input),
                class: "sr-only peer",
                r#type: "checkbox",
                value: ""
            }
            div {
                class: "relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"
            }
        }
    }
}

fn Delete(cluster_name: String, mut connections: UseConnections) -> Element {
    rsx! {
        div {
            onclick: move |_| {
                if connections.remove_cluster(&cluster_name).is_some() {
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("{} cluster has been removed!", cluster_name)));
                } else {
                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("Could not find `{}` cluster!", cluster_name)));
                }
            },
            title: "Delete",
            class: "cursor-pointer w-8",
            {BinSvg()}
        }
    }
}

#[component]
fn AddClusterModal(mut show_add_cluster_modal: Signal<bool>, mut connections: UseConnections) -> Element {
    #[derive(Debug, Default)]
    struct AddCluster {
        name: String,
        endpoint: String,
        network: MyCluster,
    }

    let mut add_cluster = use_signal(|| AddCluster::default());
    let should_show_button = !add_cluster.read().name.is_empty() && !add_cluster.read().endpoint.is_empty();

    if *show_add_cluster_modal.read() {
        rsx! {
            div {
                class: "fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center",
                div {
                    class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-gray-200 dark:bg-[#10141f] rounded-3xl",
                    div {
                        class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick: move |_| show_add_cluster_modal.set(false),
                            class: "wallet-adapter-modal-button-close w-[25px] items-center justify-center",
                            {CloseSvg()}
                        }
                    }
                    div {
                        class: "flex w-4/5 rounded-xl min-h-[40vh] p-5 mb-10 items-start justify-center flex-col",
                        label {
                            class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for: "cluster-name",
                            "What would you like to call your cluster?"
                        }
                        div {
                            class: "flex w-full mb-10",
                            span {
                                class: "w-[40px] inline-flex items-center px-3 text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                {ClusterName()}
                            }
                            input {
                                oninput: move |event| {
                                    add_cluster.write().name = event.data.value();
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "cluster-name",
                                placeholder: "Rising Sun",
                                r#type: "text",
                                required: true,
                            }
                        }
                        label {
                            class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for: "endpoint",
                            "What URL & Custom port will you reach your cluster?"
                        }
                        div {
                            class: "flex w-full",
                            span {
                                class: "w-[40px] inline-flex items-center px-3 text-lg text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                {LinkSvg()}
                            }
                            input {
                                oninput: move |event| {
                                    let data = event.data.value();
                                    if validate_url(&data) {
                                        add_cluster.write().endpoint = data;
                                    }
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "endpoint",
                                placeholder: "URL Endpoint, e.g., http://localhost:8899",
                                r#type: "url",
                                required: true,
                            }
                        }
                        label {
                            class: "block mb-2 text-sm mt-5 font-medium text-gray-900 dark:text-white",
                            r#for: "network",
                            "Network"
                        }
                        div {
                            class: "flex w-full",
                            span {
                                class: "w-[40px] inline-flex items-center px-3 bg-gray-200 border border-gray-300 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                {ClustersSvg()}
                            }
                            select {
                                onchange: move |event| {
                                    let network: MyCluster = event.data.value().as_str().try_into().unwrap_or_default();
                                    add_cluster.write().network = network;
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "network",
                                name: "network",
                                required: true,
                                for network in [MyCluster::MainNet, MyCluster::TestNet, MyCluster::DevNet, MyCluster::LocalNet] {
                                    option {
                                        key: "{network.to_string()}",
                                        value: "{network.to_string()}",
                                        {network.to_string()}
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex w-full items-center justify-center p-5 mt-5",
                            if should_show_button {
                                button {
                                    onclick: move |_| {
                                        let adapter_cluster = AdapterCluster::new()
                                            .add_name(add_cluster.read().name.as_str())
                                            .add_endpoint(add_cluster.read().endpoint.as_str())
                                            .add_cluster(add_cluster.read().network);
                                        let name = adapter_cluster.name().to_string();
                                        match connections.add_cluster(adapter_cluster) {
                                            Ok(()) => {
                                                GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("Added `{}` cluster!", name)));
                                                show_add_cluster_modal.set(false);
                                                add_cluster.set(AddCluster::default());
                                            }
                                            Err(error) => {
                                                GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(format!("Error adding cluster: `{}`!", error)));
                                                show_add_cluster_modal.set(false);
                                                add_cluster.set(AddCluster::default());
                                            }
                                        }
                                    },
                                    class: "bg-true-blue text-sm text-white px-5 py-2 rounded-full hover:bg-cobalt-blue",
                                    "ADD CLUSTER"
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}

fn validate_url(value: &str) -> bool {
    Url::parse(value).is_ok()
}