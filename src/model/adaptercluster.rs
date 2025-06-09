use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Serialize;
use serde::Deserialize;
use crate::model::mycluster::MyCluster;
//use crate::model::Connection;
use wallet_adapter::Cluster;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct AdapterCluster {
    pub name: String,
    pub cluster: MyCluster,
    pub endpoint: String,
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
        let name :&  str = self.name.as_str();
        let clst : MyCluster = name.try_into()
            .unwrap_or_else(|_| MyCluster::LocalNet); // Default to LocalNet if conversion fails     
            
        AdapterCluster {
            name: self.name.clone(),
            endpoint: self.endpoint.clone(),
            cluster: clst,
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


impl AdapterCluster {
    // Mask sensitive parts of the URL for display
    pub fn masked_url(&self) -> String {
        if let Some(token_start) = self.endpoint.find("?") {
            let (before_token, after_token) = self.endpoint.split_at(token_start + 6); // "token=".len() = 6
            if let Some(token_end) = after_token.find('&') {
                let (token_part, rest) = after_token.split_at(token_end);
                let masked_token = if token_part.len() > 8 {
                    format!(
                        "{}...{}",
                        &token_part[..4],
                        &token_part[token_part.len() - 4..]
                    )
                } else {
                    "*".repeat(token_part.len())
                };
                format!("{}{}{}", before_token, masked_token, rest)
            } else {
                // Token is at the end of endpoint
                let masked_token = if after_token.len() > 8 {
                    format!(
                        "{}...{}",
                        &after_token[..4],
                        &after_token[after_token.len() - 4..]
                    )
                } else {
                    "*".repeat(after_token.len())
                };
                format!("{}{}", before_token, masked_token)
            }
        } else if self.endpoint.contains("access_token=") {
            // Handle access_token parameter
            if let Some(token_start) = self.endpoint.find("access_token=") {
                let (before_token, after_token) = self.endpoint.split_at(token_start + 13); // "access_token=".len() = 13
                if let Some(token_end) = after_token.find('&') {
                    let (token_part, rest) = after_token.split_at(token_end);
                    let masked_token = if token_part.len() > 8 {
                        format!(
                            "{}...{}",
                            &token_part[..4],
                            &token_part[token_part.len() - 4..]
                        )
                    } else {
                        "*".repeat(token_part.len())
                    };
                    format!("{}{}{}", before_token, masked_token, rest)
                } else {
                    let masked_token = if after_token.len() > 8 {
                        format!(
                            "{}...{}",
                            &after_token[..4],
                            &after_token[after_token.len() - 4..]
                        )
                    } else {
                        "*".repeat(after_token.len())
                    };
                    format!("{}{}", before_token, masked_token)
                }
            } else {
                self.endpoint.clone()
            }
        } else if self.endpoint.contains("://") && self.endpoint.matches(':').count() >= 2 {
            // Handle endpoints with embedded credentials (user:pass@host)
            if let Some(at_pos) = self.endpoint.find('@') {
                if let Some(scheme_end) = self.endpoint.find("://") {
                    let scheme_part = &self.endpoint[..scheme_end + 3];
                    let after_at = &self.endpoint[at_pos..];
                    format!("{}***{}", scheme_part, after_at)
                } else {
                    self.endpoint.clone()
                }
            } else {
                self.endpoint.clone()
            }
        } else {
            self.endpoint.clone()
        }
    }
}

