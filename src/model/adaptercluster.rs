use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Serialize;
use serde::Deserialize;
use crate::model::mycluster::MyCluster;
use crate::model::Connection;
use wallet_adapter::Cluster;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct AdapterCluster {
    name: String,
    cluster: MyCluster,
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

    pub fn add_cluster(mut self, cluster: MyCluster) -> Self {
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
        self.cluster.toCluster()
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

//    pub fn identifier(&self) -> String {
//         self.to_string()
//    }

     pub fn query_string(&self) -> String {
         if self.name.as_bytes() == self.cluster.to_string().as_bytes()
             && self.cluster != MyCluster::LocalNet
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
            cluster: MyCluster::DevNet,
            endpoint: MyCluster::DevNet.endpoint().to_string(),
        }
    }

    pub fn mainnet() -> Self {
        AdapterCluster {
            name: "mainnet".to_string(),
            cluster: MyCluster::MainNet,
            endpoint: MyCluster::MainNet.endpoint().to_string(),
        }
    }

    pub fn testnet() -> Self {
        AdapterCluster {
            name: "testnet".to_string(),
            cluster: MyCluster::TestNet,
            endpoint: MyCluster::TestNet.endpoint().to_string(),
        }
    }

    pub fn localnet() -> Self {
        AdapterCluster {
            name: "localnet".to_string(),
            cluster: MyCluster::LocalNet,
            endpoint: MyCluster::LocalNet.endpoint().to_string(),
        }
    }

    // Mask sensitive parts of the endpoint for display
    pub fn masked_endpoint(&self) -> String {
        Connection {
            name: self.name.clone(),
            url: self.endpoint.clone(),
            cluster_name: self.name.clone(),
        }
        .masked_url()
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
