use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-bitcoin/.cargo"]
pub struct OurVendorRustBitcoinCargoExtractor;
