/// Error handling enum
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Error)]
pub enum MyWalletError {
    #[error("Unsupported")]
    UnsupportedChain(String),
}
