use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca/rsa-4096"]
pub struct OurVendorRustlsTestCaRsa4096Extractor;
