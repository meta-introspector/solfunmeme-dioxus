use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca"]
pub struct OurVendorRustlsTestCaExtractor;
