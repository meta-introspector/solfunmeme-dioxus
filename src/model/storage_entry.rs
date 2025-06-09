// Define the storage entry for persistent state
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
///use crate::model::connection::Connection;
use crate::model::adaptercluster::AdapterCluster;

#[derive(Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: String,
    //pub connections: HashMap<String, AdapterCluster>, // Keyed by name for uniqueness
    //pub clusters: Vec<AdapterCluster>,            // Store clusters separately
    pub entries: Vec<AdapterCluster>,            // Store clusters separately
}
