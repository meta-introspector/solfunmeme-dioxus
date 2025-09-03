use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca/rsa-3072"]
pub struct OurVendorRustlsTestCaRsa3072Extractor;
