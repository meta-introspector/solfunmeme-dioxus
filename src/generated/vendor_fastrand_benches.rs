use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/fastrand/benches"]
pub struct OurVendorFastrandBenchesExtractor;
