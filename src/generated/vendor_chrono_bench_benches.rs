use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/chrono/bench/benches"]
pub struct OurVendorChronoBenchBenchesExtractor;
