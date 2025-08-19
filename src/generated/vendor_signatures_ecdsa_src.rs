use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/ecdsa/src"]
pub struct OurVendorSignaturesEcdsaSrcExtractor;
