use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-bitcoin/fuzz"]
pub struct OurVendorRustBitcoinFuzzExtractor;
