use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio/src"]
pub struct OurVendorTokioTokioSrcExtractor;
