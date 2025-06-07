use dioxus::prelude::*;
use crate::model::{Connection, UseConnections, use_connections};
use crate::views::connection_filter;
use crate::views::connection_management_section;

fn list_connections(
    // connections: &UseConnections,
    //filtered_connections: &Vec<Connection>, //, connections: &UseConnections
) -> Element {
    let connections = use_connections("app_data");
    rsx! {
        div { class: "divide-y divide-gray-200 dark:divide-gray-700",

              
              for conn in connections.get_all_connections() {
		  //for conn in filtered_connections {
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
                                            let conn_name = conn.name.clone();
                                            let mut connections = use_connections("app_data");
                                            connections.remove_connection(&conn_name)
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
                  {  list_connections()      }
              }
        }
    }
}
