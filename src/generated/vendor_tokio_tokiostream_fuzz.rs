use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio-stream/fuzz"]
pub struct OurVendorTokioTokioStreamFuzzExtractor;
