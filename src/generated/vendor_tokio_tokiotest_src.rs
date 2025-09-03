use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tokio-test/src"]
pub struct OurVendorTokioTokioTestSrcExtractor;
