/// Solana Clusters
use serde::{Deserialize, Serialize};
use wallet_adapter::WalletError;
#[derive(
    Debug, PartialEq, Eq, Default, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
pub enum MyCluster {
    /// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
    MainNet,
    /// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
    #[default]
    DevNet,
    /// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
    TestNet,
    /// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
    LocalNet,
    SolfunmemeNet,
}

/// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
pub const MAINNET_IDENTIFIER: &str = "solana:mainnet";
/// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
pub const DEVNET_IDENTIFIER: &str = "solana:devnet";
/// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
pub const TESTNET_IDENTIFIER: &str = "solana:testnet";
/// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
pub const LOCALNET_IDENTIFIER: &str = "solana:localnet";
//pub const PRIVATENET_IDENTIFIER: &str = "solana:solfunmeme";

/// Solana Mainnet cluster,  [https://api.mainnet-beta.solana.com](https://api.mainnet-beta.solana.com)
pub const MAINNET: &str = "mainnet";
/// Solana Devnet cluster, e.g. [https://api.devnet.solana.com](https://api.devnet.solana.com)
pub const DEVNET: &str = "devnet";
/// Solana Testnet cluster, e.g. [https://api.testnet.solana.com](https://api.testnet.solana.com)
pub const TESTNET: &str = "testnet";
/// Solana Localnet cluster, e.g. [http://localhost:8899](http://localhost:8899)
pub const LOCALNET: &str = "localnet";
pub const SOLFUNMEMENET: &str = "solfunmeme";

/// Solana Mainnet cluster
pub const MAINNET_ENDPOINT: &str = "https://api.mainnet-beta.solana.com";
/// Solana Devnet cluster
pub const DEVNET_ENDPOINT: &str = "https://api.devnet.solana.com";
/// Solana Testnet cluster
pub const TESTNET_ENDPOINT: &str = "https://api.testnet.solana.com";
/// Solana Localnet cluster
pub const LOCALNET_ENDPOINT: &str = "https://localhost:8899";
pub const SOLFUNMEMENET_ENDPOINT: &str = "https://solana.solfunmeme.com/validator/";

impl MyCluster {
    pub fn toCluster(&self) -> wallet_adapter::Cluster {
        match self {
            MyCluster::MainNet => wallet_adapter::Cluster::MainNet,
            MyCluster::DevNet => wallet_adapter::Cluster::DevNet,
            MyCluster::TestNet => wallet_adapter::Cluster::TestNet,
            MyCluster::LocalNet => wallet_adapter::Cluster::LocalNet,
            MyCluster::SolfunmemeNet => wallet_adapter::Cluster::LocalNet,
        }
    }   
    /// A Solana endpoint URI
    pub fn endpoint(&self) -> &str {
        match self {
            MyCluster::MainNet => MAINNET_ENDPOINT,
            MyCluster::DevNet => DEVNET_ENDPOINT,
            MyCluster::TestNet => TESTNET_ENDPOINT,
            MyCluster::LocalNet => LOCALNET_ENDPOINT,
	    MyCluster::SolfunmemeNet => SOLFUNMEMENET_ENDPOINT,
        }
    }

    /// A Solana cluster identifier as a &str
    pub fn display(&self) -> &str {
        match self {
            MyCluster::MainNet => MAINNET,
            MyCluster::DevNet => DEVNET,
            MyCluster::TestNet => TESTNET,
            MyCluster::LocalNet => LOCALNET,
	    MyCluster::SolfunmemeNet => SOLFUNMEMENET,
        }
    }


    
}

impl core::fmt::Display for MyCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}




impl TryFrom<&str> for MyCluster {
    type Error = WalletError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cluster = match value {
            MAINNET_IDENTIFIER => Self::MainNet,
            DEVNET_IDENTIFIER => Self::DevNet,
            TESTNET_IDENTIFIER => Self::TestNet,
            LOCALNET_IDENTIFIER => Self::LocalNet,
            MAINNET_ENDPOINT => Self::MainNet,
            DEVNET_ENDPOINT => Self::DevNet,
            TESTNET_ENDPOINT => Self::TestNet,
            LOCALNET_ENDPOINT => Self::LocalNet,
            MAINNET => Self::MainNet,
            DEVNET => Self::DevNet,
            TESTNET => Self::TestNet,
            LOCALNET => Self::LocalNet,
            _ => return Err(WalletError::UnsupportedChain(value.to_string())),
        };

        Ok(cluster)
    }
}
