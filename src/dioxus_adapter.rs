use dioxus::prelude::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use wallet_adapter::{
    web_sys::{self, Window},
    Cluster,
};

pub(crate) static WINDOW: GlobalSignal<Window> =
    Signal::global(|| web_sys::window().expect("Unable to find Window"));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub(crate) struct ClusterStore {
    clusters: Vec<AdapterCluster>,
    active_cluster: AdapterCluster,
}

impl ClusterStore {
    pub fn new(clusters: Vec<AdapterCluster>) -> Self {
        Self {
            clusters,
            active_cluster: AdapterCluster::default(),
        }
    }

    pub fn get_clusters(&self) -> &[AdapterCluster] {
        self.clusters.as_slice()
    }

    pub fn add_cluster(&mut self, cluster: AdapterCluster) -> Result<&mut Self, String> {
        let cluster_exists = self.clusters.iter().any(|inner_cluster| {
            inner_cluster.name.as_bytes() == cluster.name.as_bytes()
                || inner_cluster.endpoint.as_bytes() == cluster.endpoint.as_bytes()
        });

        if cluster_exists {
            Err(String::from(
                "Cluster exists, make sure endpoint or name are not the same",
            ))
        } else {
            self.clusters.push(cluster);
            Ok(self)
        }
    }

    pub fn set_active_cluster(&mut self, cluster: AdapterCluster) -> &mut Self {
        self.active_cluster = cluster;

        self
    }

    pub fn active_cluster(&self) -> &AdapterCluster {
        &self.active_cluster
    }

    pub fn add_clusters(&mut self, clusters: Vec<AdapterCluster>) -> Result<(), String> {
        clusters.into_iter().try_for_each(|cluster| {
            self.add_cluster(cluster)?;

            Ok::<(), String>(())
        })
    }

    pub fn get_cluster(&self, name: &str) -> Option<&AdapterCluster> {
        self.clusters.iter().find(|cluster| cluster.name == name)
    }

    pub fn remove_cluster(&mut self, cluster_name: &str) -> Option<AdapterCluster> {
        self.clusters
            .iter()
            .position(|current_cluster| current_cluster.name.as_bytes() == cluster_name.as_bytes())
            .map(|index| self.clusters.remove(index))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct AdapterCluster {
    name: String,
    cluster: Cluster,
    endpoint: String,
}

impl AdapterCluster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_name(mut self, name: &str) -> Self {
        self.name = name.to_string();

        self
    }

    pub fn add_cluster(mut self, cluster: Cluster) -> Self {
        self.cluster = cluster;

        self
    }

    pub fn add_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();

        self
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn cluster(&self) -> Cluster {
        self.cluster
    }
    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub fn identifier(&self) -> String {
        self.to_string()
    }

    pub fn query_string(&self) -> String {
        if self.name.as_bytes() == self.cluster.to_string().as_bytes()
            && self.cluster != Cluster::LocalNet
        {
            String::new() + "?cluster=" + self.cluster.to_string().as_str()
        } else {
            String::new()
                + "?cluster=custom&customUrl="
                + utf8_percent_encode(self.endpoint.as_str(), NON_ALPHANUMERIC)
                    .to_string()
                    .as_str()
        }
    }

    pub fn devnet() -> Self {
        AdapterCluster {
            name: "devnet".to_string(),
            cluster: Cluster::DevNet,
            endpoint: Cluster::DevNet.endpoint().to_string(),
        }
    }

    pub fn mainnet() -> Self {
        AdapterCluster {
            name: "mainnet".to_string(),
            cluster: Cluster::MainNet,
            endpoint: Cluster::MainNet.endpoint().to_string(),
        }
    }

    pub fn testnet() -> Self {
        AdapterCluster {
            name: "testnet".to_string(),
            cluster: Cluster::TestNet,
            endpoint: Cluster::TestNet.endpoint().to_string(),
        }
    }

    pub fn localnet() -> Self {
        AdapterCluster {
            name: "localnet".to_string(),
            cluster: Cluster::LocalNet,
            endpoint: Cluster::LocalNet.endpoint().to_string(),
        }
    }
}

impl Default for AdapterCluster {
    fn default() -> Self {
        Self::devnet()
    }
}

impl std::fmt::Display for AdapterCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cluster.display())
    }
}
