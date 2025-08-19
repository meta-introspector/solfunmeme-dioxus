use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/stress-test"]
pub struct OurVendorTokioStressTestExtractor;
