use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/block-modes/cbc/src"]
pub struct OurVendorBlockModesCbcSrcExtractor;
