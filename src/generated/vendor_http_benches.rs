use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/http/benches"]
pub struct OurVendorHttpBenchesExtractor;
