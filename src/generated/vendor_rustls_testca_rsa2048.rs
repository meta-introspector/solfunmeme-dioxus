use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca/rsa-2048"]
pub struct OurVendorRustlsTestCaRsa2048Extractor;
