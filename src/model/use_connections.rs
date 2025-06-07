use dioxus::prelude::*;

use crate::model::storage_entry::StorageEntry;
use crate::model::adaptercluster::AdapterCluster;
use crate::model::connection::Connection;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use wallet_adapter::Cluster;
//storage_entry
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

