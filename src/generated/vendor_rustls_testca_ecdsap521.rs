use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca/ecdsa-p521"]
pub struct OurVendorRustlsTestCaEcdsaP521Extractor;
