use dioxus::prelude::*;
use crate::model::UseConnections;
use crate::views::connection_list;
use crate::views::connection_management_section;
use crate::views::cluster_management_section;
use crate::model::Connection;
use crate::model::use_connections;
use crate::model::AdapterCluster;
use crate::model::MyCluster;
use crate::model::ClusterNetState;

pub fn connection_filter(
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
