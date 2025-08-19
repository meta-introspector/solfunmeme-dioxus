use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ndarray/benches"]
pub struct OurVendorNdarrayBenchesExtractor;
