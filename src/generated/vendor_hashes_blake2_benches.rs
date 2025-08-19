use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/blake2/benches"]
pub struct OurVendorHashesBlake2BenchesExtractor;
