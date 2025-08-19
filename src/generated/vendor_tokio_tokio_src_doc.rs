use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio/src/doc"]
pub struct OurVendorTokioTokioSrcDocExtractor;
