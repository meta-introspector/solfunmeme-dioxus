use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/sha3/benches"]
pub struct OurVendorHashesSha3BenchesExtractor;
