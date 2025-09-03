use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/httparse/benches"]
pub struct OurVendorHttparseBenchesExtractor;
