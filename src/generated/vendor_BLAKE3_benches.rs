use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/BLAKE3/benches"]
pub struct OurVendorBLAKE3BenchesExtractor;
