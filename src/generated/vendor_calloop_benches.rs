use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/calloop/benches"]
pub struct OurVendorCalloopBenchesExtractor;
