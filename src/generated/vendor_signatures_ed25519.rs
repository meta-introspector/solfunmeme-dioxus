use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/ed25519"]
pub struct OurVendorSignaturesEd25519Extractor;
