use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;
use wallet_adapter::{
    Cluster,
};

use crate::{connection_management_section, model::use_connections, views::{cluster_management_section, connection_filter, connection_list}};

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
              h2 { class: "text-3xl font-bold mb-8 text-gray-800 dark:text-white", "Manage Connections & Clusters" }
              { cluster_management_section(new_cluster_name, new_cluster_type,new_cluster_endpoint,show_cluster_form,connections )}
              { connection_management_section(cluster_names, new_connection_name, new_connection_url, selected_cluster_for_connection ) }
              { connection_filter( filter_options,  selected_cluster_filter ) }
              { connection_list(&filtered_connections,  selected_cluster_filter) }                        
        }
    }
}
