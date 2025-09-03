use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/server"]
pub struct OurVendorHyperSrcServerExtractor;
