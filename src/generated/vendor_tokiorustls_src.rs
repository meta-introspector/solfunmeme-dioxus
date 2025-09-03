use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio-rustls/src"]
pub struct OurVendorTokioRustlsSrcExtractor;
