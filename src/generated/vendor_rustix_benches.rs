use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/benches"]
pub struct OurVendorRustixBenchesExtractor;
