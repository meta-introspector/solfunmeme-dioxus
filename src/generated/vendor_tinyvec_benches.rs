use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tinyvec/benches"]
pub struct OurVendorTinyvecBenchesExtractor;
