use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio-rustls"]
pub struct OurVendorTokioRustlsExtractor;
