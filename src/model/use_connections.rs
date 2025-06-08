use dioxus::prelude::*;

use crate::model::storage_entry::StorageEntry;
use crate::model::adaptercluster::AdapterCluster;
use crate::model::connection::Connection;

use gloo_storage::{LocalStorage, Storage};
//use url::Url;
use std::collections::HashMap;

//storage_entry
// Custom hook to manage persistent connections and clusters
#[derive(Clone, Copy, PartialEq)]
pub struct UseConnections {
    inner: Signal<StorageEntry>,
    active_cluster: Signal<String>,
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

    pub fn get_cluster(&self, name: &str) -> Option<AdapterCluster> {
        self.inner.read().clusters.iter().find(|c| c.name() == name).cloned()
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

    pub fn remove_connection(&mut self, name: &str) {
        let mut inner = self.inner.write();
        inner.connections.remove(name);
        LocalStorage::set(&format!("{}_connections", inner.key), &inner.connections)
            .expect("Failed to save connections to LocalStorage");
    }

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



