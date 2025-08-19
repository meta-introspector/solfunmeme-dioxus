use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ndarray-stats/benches"]
pub struct OurVendorNdarrayStatsBenchesExtractor;
