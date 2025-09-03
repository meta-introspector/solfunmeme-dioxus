use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio/src/loom"]
pub struct OurVendorTokioTokioSrcLoomExtractor;
