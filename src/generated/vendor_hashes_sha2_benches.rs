use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/sha2/benches"]
pub struct OurVendorHashesSha2BenchesExtractor;
