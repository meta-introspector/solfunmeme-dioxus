use dioxus::prelude::*;
use wallet_adapter::Cluster;
use crate::model::storage_entry::StorageEntry;
use crate::model::adaptercluster::AdapterCluster;
use gloo_storage::{LocalStorage, Storage};
//use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub struct UseConnections {
    inner: Signal<StorageEntry>,
    active_entry: Signal<String>, // Renamed to reflect unified entries
}

impl UseConnections {
    // Get all entries (replaces get_all_connections and get_all_clusters)
    pub fn get_all_entries(&self) -> Vec<AdapterCluster> {
        self.inner.read().entries.clone()
    }

    // Get entries by name (replaces get_connections_by_cluster)
    pub fn get_entries_by_name(&self, name: &str) -> Vec<AdapterCluster> {
        self.inner
            .read()
            .entries
            .iter()
            .filter(|entry| entry.name() == name)
            .cloned()
            .collect()
    }

    // Get a specific entry by name (replaces get_cluster)
    pub fn get_entry(&self, name: &str) -> Option<AdapterCluster> {
        self.inner.read().entries.iter().find(|e| e.name() == name).cloned()
    }

    // Get all unique entry names, sorted (replaces get_cluster_names)
    pub fn get_entry_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .inner
            .read()
            .entries
            .iter()
            .map(|entry| entry.name().to_string())
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect();
        names.sort();
        names
    }

    // Add a new entry (replaces add_connection and add_cluster)
    pub fn add_entry(&mut self, entry: AdapterCluster) -> Result<(), String> {
        let mut inner = self.inner.write();

        // Check for duplicate name or endpoint
        if inner.entries.iter().any(|existing| {
            existing.name() == entry.name() || existing.endpoint() == entry.endpoint()
        }) {
            return Err("Entry exists with the same name or endpoint".to_string());
        }

        inner.entries.push(entry);
        LocalStorage::set(&format!("{}_entries", inner.key), &inner.entries)
            .expect("Failed to save entries to LocalStorage");
        Ok(())
    }

    // Remove an entry by name (replaces remove_connection and remove_cluster)
    pub fn remove_entry(&mut self, name: &str) -> Option<AdapterCluster> {
        let mut inner = self.inner.write();

        // Find and remove the entry
        let position = inner.entries.iter().position(|entry| entry.name() == name)?;
        let removed_entry = inner.entries.remove(position);

        // Save updated entries
        LocalStorage::set(&format!("{}_entries", inner.key), &inner.entries)
            .expect("Failed to save entries to LocalStorage");

        // Update active entry if the removed one was active
        if *self.active_entry.read() == name {
            let new_active = inner.entries.first().map(|e| e.name().to_string()).unwrap_or_default();
            self.active_entry.set(new_active.clone());
            LocalStorage::set(&format!("{}_active_entry", inner.key), &new_active)
                .expect("Failed to save active entry");
        }

        Some(removed_entry)
    }

    // Set the active entry (replaces set_active_cluster)
    pub fn set_active_entry(&mut self, name: String) {
        self.active_entry.set(name.clone());
        LocalStorage::set(&format!("{}_active_entry", self.inner.read().key), &name)
            .expect("Failed to save active entry");
    }

    // Get the active entry name (replaces active_cluster)
    pub fn active_entry(&self) -> String {
        self.active_entry.read().clone()
    }

    // Get the active entry object (replaces active_cluster_object)
    pub fn active_entry_object(&self) -> AdapterCluster {
        let active_name = self.active_entry.read().clone();
        self.get_entry(&active_name).unwrap()
    }

    pub fn supports_airdrop(&self, active_cluster_name: &str) -> bool {

        let active_entry = self.get_entry(&active_cluster_name).unwrap();
        active_entry.cluster() != Cluster::MainNet
    }

}

pub fn use_connections(key: impl ToString) -> UseConnections {
    let key = key.to_string();
    let key_for_state = key.clone();
    let key_for_active = key.clone();

    let state = use_signal(move || {
        let entries: Vec<AdapterCluster> = LocalStorage::get(&format!("{}_entries", &key_for_state))
            .ok()
            .unwrap_or_else(|| {
                vec![
                    AdapterCluster::devnet(),
                    AdapterCluster::testnet(),
                    AdapterCluster::mainnet(),
                    AdapterCluster::localnet(),
                ]
            });
        StorageEntry {
            key: key_for_state.clone(),
            entries,
        }
    });

    let active_entry = use_signal(move || {
        LocalStorage::get(&format!("{}_active_entry", &key_for_active))
            .ok()
            .unwrap_or_else(|| {
                state.read().entries.first().map(|e| e.name().to_string()).unwrap_or_default()
            })
    });

    UseConnections {
        inner: state,
        active_entry,
    }
}