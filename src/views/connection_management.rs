use dioxus::prelude::*;
use crate::model::{use_connections, AdapterCluster, MyCluster};

pub(crate) fn connection_management_section(
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
                    let mc :MyCluster = MyCluster::try_from(selected_cluster_for_connection().as_str()).unwrap_or(MyCluster::MainNet);
                    let connection = AdapterCluster {
                          name: new_connection_name().clone(),
                          endpoint: new_connection_url().clone(),
                          cluster: mc
                      };
                      if !connection.name.is_empty() && !connection.endpoint.is_empty()  {
                          connections.add_entry(connection);
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
