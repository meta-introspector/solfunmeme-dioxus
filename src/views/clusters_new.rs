use dioxus::prelude::*;

use crate::{
    //utils::{get_cluster_svg, trunk_cluster_name},
    AdapterCluster,
    //BinSvg, CheckSvg, 
    CloseSvg, ClusterName, ClustersSvg, LinkSvg,
    NotificationInfo, CLUSTER_STORAGE, GLOBAL_MESSAGE,
    views::clusters::ClusterInfo
};

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wallet_adapter::Cluster;
use serde::de::DeserializeOwned;

// #[component]
// pub fn Explainer<'a>(
//     cx: Scope<'a>,
//     invert: bool,
//     title: &'static str,
//     content: Element,
//     flasher: Element,
// ) -> Element {
//     // pt-5 sm:pt-24 lg:pt-24

//     let mut right = rsx! {
//         div { class: "relative w-1/2", {flasher} }
//     };

//     let align = match invert {
//         true => "mr-auto ml-16",
//         false => "ml-auto mr-16",
//     };

//     let mut left = rsx! {
//         div { class: "relative w-1/2 {align} max-w-md leading-8",
//             h2 { class: "mb-6 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-heading font-mono font-bold",
//                 "{title}"
//             }
//             {content}
//         }
//     };

//     if *invert {
//         std::mem::swap(&mut left, &mut right);
//     }

//     rsx! {
//         div { class: "flex flex-wrap items-center dark:text-white py-16 border-t font-light",
//             {left}
//             {right}
//         }
//     }
// }


// Persistent storage hook
#[derive(Clone, Copy)]
pub struct UsePersistent<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

#[allow(clippy::needless_return)]
pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UsePersistent<T> {
    let state = use_signal(move || {
        let key = key.to_string();
        let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(init);
        StorageEntry { key, value }
    });
    UsePersistent { inner: state }
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        LocalStorage::set(inner.key.as_str(), &value);
        inner.value = value;
    }
}

// Store cluster endpoints in a HashMap
#[derive(Serialize, Deserialize, Clone, Default)]
struct ClusterEndpoints(HashMap<String, String>);

// Get cluster endpoint from storage
pub fn get_prod_url(cx: &ScopeState, name: &str) -> String {
    let persistent = use_persistent("clusters", || ClusterEndpoints(HashMap::new()));
    let endpoints = persistent.get();
    endpoints
        .0
        .get(name)
        .cloned()
        .unwrap_or("https://api.mainnet-beta.solana.com".to_string())
}

// Set cluster endpoint in storage
pub fn set_url(cx: &ScopeState, name: &str, value: &str) {
    let mut persistent = use_persistent("clusters", || ClusterEndpoints(HashMap::new()));
    let mut endpoints = persistent.get();
    endpoints.0.insert(name.to_string(), value.to_string());
    persistent.set(endpoints);
}

// Initialize CLUSTER_STORAGE with endpoints from local storage
fn init_clusters_from_storage(cx: &ScopeState) {
    let persistent = use_persistent("clusters", || ClusterEndpoints(HashMap::new()));
    let endpoints = persistent.get();

    for (name, endpoint) in endpoints.0 {
        // Check if cluster exists in CLUSTER_STORAGE
        let existing_cluster = CLUSTER_STORAGE.read().get_cluster(&name).cloned();
        match existing_cluster {
            Some(cluster) => {
                // Update endpoint if different
                if endpoint != cluster.endpoint() {
                    let mut new_cluster = AdapterCluster::new()
                        .add_name(&name)
                        .add_endpoint(&endpoint)
                        .add_cluster(cluster.cluster());
                    if let Err(error) = CLUSTER_STORAGE.write().add_cluster(new_cluster) {
                        GLOBAL_MESSAGE
                            .write()
                            .push_back(NotificationInfo::new(format!(
                                "Error updating cluster `{name}`: {error}"
                            )));
                    }
                }
            }
            None => {
                // Add new cluster with default network (e.g., Mainnet)
                let new_cluster = AdapterCluster::new()
                    .add_name(&name)
                    .add_endpoint(&endpoint)
                    .add_cluster(Cluster::MainNet); // Adjust default as needed
                if let Err(error) = CLUSTER_STORAGE.write().add_cluster(new_cluster) {
                    GLOBAL_MESSAGE
                        .write()
                        .push_back(NotificationInfo::new(format!(
                            "Error adding cluster `{name}`: {error}"
                        )));
                }
            }
        }
    }
}

//#[component]
//pub fn Clusters<'a> (cx: & 'a ScopeState) -> Element {
//pub fn Clusters (cx: & ScopeState) -> Element {
// pub fn Clusters2 (cx: Scope) -> Element {
// //pub fn ClustersNew2 () -> Element {   
//     // Run initialization on component mount
//     // use_effect(move || {
//     //    //init_clusters_from_storage(cx);
//     //    // || () // Cleanup function (empty for now)
//     // });
// }



// #[component]
// pub fn Explainer<'a>(
//     cx: Scope<'a>,
//     invert: bool,
//     title: &'static str,
//     content: Element,
//     flasher: Element,
// ) -> Element {
//     // pt-5 sm:pt-24 lg:pt-24
// }

#[component]
//pub fn Clusters<'a> (cx: & 'a ScopeState) -> Element {
//pub fn Clusters (cx: & ScopeState) -> Element {
pub fn ClustersNew () -> Element {
    let mut show_add_cluster_modal = use_signal(|| false);

    // Run initialization on component mount
    // use_effect(move || {
    //    //init_clusters_from_storage(cx);
    //    // || () // Cleanup function (empty for now)
    // });

    rsx! {
        div { class: "flex w-full flex-col justify-start p-10 items-center",
            div { class: "flex flex-col w-full items-center justify-center text-4xl",
                span { class: "flex w-[100px]", {ClustersSvg()} }, "Clusters"
                div { class: "text-xl", "Manage your Solana endpoints" }
                button {
                    onclick: move |_| {
                        show_add_cluster_modal.set(true);
                    },
                    class: "bg-true-blue text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue",
                    "ADD CLUSTER"
                }
                div { class: "flex flex-wrap w-full items-stretch justify-center gap-4 mt-20",
                    ClusterInfo {}
                }
            }
        }
        AddClusterModal { show_add_cluster_modal }
    }
}

#[component]
fn AddClusterModal(mut show_add_cluster_modal: Signal<bool>) -> Element {
    #[derive(Debug, Default)]
    struct AddCluster {
        name: String,
        endpoint: String,
        network: Cluster,
    }

    let mut add_cluster = use_signal(|| AddCluster::default());
    let should_show_button =
        !add_cluster.read().name.is_empty() && !add_cluster.read().endpoint.is_empty();

    if *show_add_cluster_modal.read() {
        rsx! {
            div {
                class: "fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center",
                div { class: "flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-gray-200 dark:bg-[#10141f] rounded-3xl",
                    div { class: "flex w-full justify-end items-center p-5",
                        button {
                            onclick: move |_| {
                                show_add_cluster_modal.set(false);
                                add_cluster.set(AddCluster::default());
                            },
                            class: "wallet-adapter-modal-button-close w-[25px] items-center justify-center",
                            { CloseSvg() }
                        }
                    }
                    div { class: "flex w-4/5 rounded-xl min-h-[40vh] p-5 mb-10 items-start justify-center flex-col",
                        label {
                            class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for: "cluster-name",
                            "What would you like to call your cluster?"
                        }
                        div { class: "flex w-full mb-10",
                            span { class: "w-[40px] inline-flex items-center px-3 text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                { ClusterName() }
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
                            "What URL & custom port will you reach your cluster?"
                        }
                        div { class: "flex w-full",
                            span { class: "w-[40px] inline-flex items-center px-3 text-lg text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                { LinkSvg() }
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
                        div { class: "flex w-full",
                            span { class: "w-[40px] inline-flex items-center px-3 bg-gray-200 border border-gray-300 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600",
                                { ClustersSvg() }
                            }
                            select {
                                onchange: move |event| {
                                    let network: Cluster = event.data.value().as_str().try_into().expect(
                                        "Invalid cluster selected"
                                    );
                                    add_cluster.write().network = network;
                                },
                                class: "rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id: "network",
                                name: "network",
                                required: true,
                                for cluster in CLUSTER_STORAGE.read().get_clusters() {
                                    option {
                                        key: "{cluster.name()}",
                                        value: "{cluster.identifier()}",
                                        "{cluster.name()}"
                                    }
                                }
                            }
                        }
                        div { class: "flex w-full items-center justify-center p-5 mt-5",
                            if should_show_button {
                                button {
                                    onclick: move |_| {
                                        let adapter_cluster = AdapterCluster::new()
                                            .add_name(add_cluster.read().name.as_str())
                                            .add_endpoint(add_cluster.read().endpoint.as_str())
                                            .add_cluster(add_cluster.read().network);

                                        let name = adapter_cluster.name().to_string();
                                        //set_url(cx, &name, add_cluster.read().endpoint.as_str());

                                        if let Err(error) = CLUSTER_STORAGE.write().add_cluster(adapter_cluster) {
                                            GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(
                                                format!("Error adding cluster `{name}`: {error}")
                                            ));
                                        } else {
                                            GLOBAL_MESSAGE.write().push_back(NotificationInfo::new(
                                                format!("Added `{name}` cluster!")
                                            ));
                                        }

                                        show_add_cluster_modal.set(false);
                                        add_cluster.set(AddCluster::default());
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
           rsx! {
            div{class:"flex w-full flex-col justify-start p-10 items-center",
            "error"
           }
        }
    }
}

fn validate_url(value: &str) -> bool {
    let scheme_exists = value.starts_with("http://") || value.starts_with("https://");
    scheme_exists && value.len() > 8
}
