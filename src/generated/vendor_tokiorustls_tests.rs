use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio-rustls/tests"]
pub struct OurVendorTokioRustlsTestsExtractor;
