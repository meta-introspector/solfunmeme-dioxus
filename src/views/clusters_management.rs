use dioxus::prelude::*;
use wallet_adapter::Cluster;


use crate::model::{AdapterCluster, MyCluster, UseConnections};
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
