use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/quote/benches"]
pub struct OurVendorQuoteBenchesExtractor;
