use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/client"]
pub struct OurVendorHyperSrcClientExtractor;
