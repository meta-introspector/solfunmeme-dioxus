use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-smallvec/fuzz"]
pub struct OurVendorRustSmallvecFuzzExtractor;
