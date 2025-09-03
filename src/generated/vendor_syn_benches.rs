use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/benches"]
pub struct OurVendorSynBenchesExtractor;
