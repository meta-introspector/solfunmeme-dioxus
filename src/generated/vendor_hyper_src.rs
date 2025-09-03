use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src"]
pub struct OurVendorHyperSrcExtractor;
