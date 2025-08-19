use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hex-conservative/.cargo"]
pub struct OurVendorHexConservativeCargoExtractor;
