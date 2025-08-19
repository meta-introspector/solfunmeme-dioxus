use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bincode/benches"]
pub struct OurVendorBincodeBenchesExtractor;
