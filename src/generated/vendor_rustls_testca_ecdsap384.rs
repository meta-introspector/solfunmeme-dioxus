use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/test-ca/ecdsa-p384"]
pub struct OurVendorRustlsTestCaEcdsaP384Extractor;
