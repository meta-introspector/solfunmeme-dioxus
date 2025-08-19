use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/bench/data"]
pub struct OurVendorRingBenchDataExtractor;
