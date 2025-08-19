use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/groestl/benches"]
pub struct OurVendorHashesGroestlBenchesExtractor;
