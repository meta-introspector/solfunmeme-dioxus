use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/ecdsa/tests"]
pub struct OurVendorSignaturesEcdsaTestsExtractor;
