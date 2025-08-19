use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/ed25519/src"]
pub struct OurVendorSignaturesEd25519SrcExtractor;
