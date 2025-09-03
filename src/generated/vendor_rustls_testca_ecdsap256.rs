use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca/ecdsa-p256"]
pub struct OurVendorRustlsTestCaEcdsaP256Extractor;
