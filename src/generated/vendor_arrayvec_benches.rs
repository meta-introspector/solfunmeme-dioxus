use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/arrayvec/benches"]
pub struct OurVendorArrayvecBenchesExtractor;
