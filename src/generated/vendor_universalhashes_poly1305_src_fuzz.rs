use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/universal-hashes/poly1305/src/fuzz"]
pub struct OurVendorUniversalHashesPoly1305SrcFuzzExtractor;
