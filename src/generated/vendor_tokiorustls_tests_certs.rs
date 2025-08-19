use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio-rustls/tests/certs"]
pub struct OurVendorTokioRustlsTestsCertsExtractor;
