use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hex-conservative/fuzz"]
pub struct OurVendorHexConservativeFuzzExtractor;
