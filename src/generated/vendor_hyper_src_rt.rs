use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/rt"]
pub struct OurVendorHyperSrcRtExtractor;
