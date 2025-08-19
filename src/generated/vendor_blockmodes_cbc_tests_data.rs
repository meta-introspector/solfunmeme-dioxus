use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/block-modes/cbc/tests/data"]
pub struct OurVendorBlockModesCbcTestsDataExtractor;
