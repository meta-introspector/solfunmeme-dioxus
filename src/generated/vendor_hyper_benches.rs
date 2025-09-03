use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/benches"]
pub struct OurVendorHyperBenchesExtractor;
