use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio/fuzz"]
pub struct OurVendorTokioTokioFuzzExtractor;
