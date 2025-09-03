use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rand/benches"]
pub struct OurVendorRandBenchesExtractor;
