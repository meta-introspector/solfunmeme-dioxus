use crate::common::{AsyncSignalManager, SignalState};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterInfo {
    pub name: String,
    pub url: String,
    pub is_active: bool,
    pub is_healthy: bool,
    pub last_health_check: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterState {
    pub clusters: HashMap<String, ClusterInfo>,
    pub active_cluster: Option<String>,
    pub last_updated: std::time::SystemTime,
}

impl Default for ClusterState {
    fn default() -> Self {
        Self {
            clusters: HashMap::new(),
            active_cluster: None,
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Cluster signal manager for managing cluster connections
pub struct ClusterManager {
    cluster_state: AsyncSignalManager<ClusterState>,
    loading_state: AsyncSignalManager<bool>,
}

impl ClusterManager {
    pub fn new() -> Self {
        Self {
            cluster_state: AsyncSignalManager::new(ClusterState::default()),
            loading_state: AsyncSignalManager::new(false),
        }
    }

    pub fn add_cluster(&mut self, name: String, url: String) {
        let mut state = self.cluster_state.get().value;
        let cluster = ClusterInfo {
            name: name.clone(),
            url,
            is_active: false,
            is_healthy: true,
            last_health_check: std::time::SystemTime::now(),
        };
        state.clusters.insert(name, cluster);
        state.last_updated = std::time::SystemTime::now();
        self.cluster_state.set_success(state);
    }

    pub fn remove_cluster(&mut self, name: &str) {
        let mut state = self.cluster_state.get().value;
        state.clusters.remove(name);
        if state.active_cluster.as_ref() == Some(&name.to_string()) {
            state.active_cluster = None;
        }
        state.last_updated = std::time::SystemTime::now();
        self.cluster_state.set_success(state);
    }

    pub fn set_active_cluster(&mut self, name: &str) {
        let mut state = self.cluster_state.get().value;
        if state.clusters.contains_key(name) {
            // Deactivate all clusters
            for cluster in state.clusters.values_mut() {
                cluster.is_active = false;
            }
            // Activate the specified cluster
            if let Some(cluster) = state.clusters.get_mut(name) {
                cluster.is_active = true;
            }
            state.active_cluster = Some(name.to_string());
            state.last_updated = std::time::SystemTime::now();
            self.cluster_state.set_success(state);
        }
    }

    pub fn update_cluster_health(&mut self, name: &str, is_healthy: bool) {
        let mut state = self.cluster_state.get().value;
        if let Some(cluster) = state.clusters.get_mut(name) {
            cluster.is_healthy = is_healthy;
            cluster.last_health_check = std::time::SystemTime::now();
            state.last_updated = std::time::SystemTime::now();
            self.cluster_state.set_success(state);
        }
    }

    pub fn get_cluster_state(&self) -> ClusterState {
        self.cluster_state.get().value
    }

    pub fn get_clusters(&self) -> HashMap<String, ClusterInfo> {
        self.cluster_state.get().value.clusters
    }

    pub fn get_active_cluster(&self) -> Option<String> {
        self.cluster_state.get().value.active_cluster.clone()
    }

    pub fn get_cluster(&self, name: &str) -> Option<ClusterInfo> {
        self.cluster_state.get().value.clusters.get(name).cloned()
    }

    pub fn subscribe_clusters(&self) -> Signal<SignalState<ClusterState>> {
        self.cluster_state.subscribe()
    }

    pub fn set_loading(&mut self, loading: bool) {
        if loading {
            self.loading_state.set_loading();
        } else {
            self.loading_state.set_success(false);
        }
    }

    pub fn is_loading(&self) -> bool {
        self.loading_state.get().value
    }

    pub fn subscribe_loading(&self) -> Signal<SignalState<bool>> {
        self.loading_state.subscribe()
    }
}

// Global cluster manager instance
pub static CLUSTER_MANAGER: Signal<ClusterManager> = Signal::new(ClusterManager::new);

// Convenience functions for global access
pub fn add_cluster(name: String, url: String) {
    let mut manager = CLUSTER_MANAGER.read().clone();
    manager.add_cluster(name, url);
    CLUSTER_MANAGER.set(manager);
}

pub fn remove_cluster(name: &str) {
    let mut manager = CLUSTER_MANAGER.read().clone();
    manager.remove_cluster(name);
    CLUSTER_MANAGER.set(manager);
}

pub fn set_active_cluster(name: &str) {
    let mut manager = CLUSTER_MANAGER.read().clone();
    manager.set_active_cluster(name);
    CLUSTER_MANAGER.set(manager);
}

pub fn update_cluster_health(name: &str, is_healthy: bool) {
    let mut manager = CLUSTER_MANAGER.read().clone();
    manager.update_cluster_health(name, is_healthy);
    CLUSTER_MANAGER.set(manager);
}

pub fn get_cluster_state() -> ClusterState {
    CLUSTER_MANAGER.read().get_cluster_state()
}

pub fn get_clusters() -> HashMap<String, ClusterInfo> {
    CLUSTER_MANAGER.read().get_clusters()
}

pub fn get_active_cluster() -> Option<String> {
    CLUSTER_MANAGER.read().get_active_cluster()
}

pub fn get_cluster(name: &str) -> Option<ClusterInfo> {
    CLUSTER_MANAGER.read().get_cluster(name)
}

pub fn subscribe_clusters() -> Signal<SignalState<ClusterState>> {
    CLUSTER_MANAGER.read().subscribe_clusters()
}

pub fn set_cluster_loading(loading: bool) {
    let mut manager = CLUSTER_MANAGER.read().clone();
    manager.set_loading(loading);
    CLUSTER_MANAGER.set(manager);
}

pub fn is_cluster_loading() -> bool {
    CLUSTER_MANAGER.read().is_loading()
}

pub fn subscribe_cluster_loading() -> Signal<SignalState<bool>> {
    CLUSTER_MANAGER.read().subscribe_loading()
} 