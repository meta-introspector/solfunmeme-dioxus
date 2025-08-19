use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/ecdsa"]
pub struct OurVendorSignaturesEcdsaExtractor;
