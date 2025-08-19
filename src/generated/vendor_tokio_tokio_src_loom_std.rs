use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio/src/loom/std"]
pub struct OurVendorTokioTokioSrcLoomStdExtractor;
