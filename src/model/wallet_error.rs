use thiserror::Error;

/// Error handling enum
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Error)]
#[allow(dead_code)]
pub enum MyWalletError {
    #[error("Unsupported")]
    UnsupportedChain(String),
}
