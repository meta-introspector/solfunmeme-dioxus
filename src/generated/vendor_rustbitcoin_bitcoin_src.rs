use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-bitcoin/bitcoin/src"]
pub struct OurVendorRustBitcoinBitcoinSrcExtractor;
