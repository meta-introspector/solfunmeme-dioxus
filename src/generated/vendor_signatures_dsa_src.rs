use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/dsa/src"]
pub struct OurVendorSignaturesDsaSrcExtractor;
