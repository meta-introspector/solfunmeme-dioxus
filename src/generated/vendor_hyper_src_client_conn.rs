use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/client/conn"]
pub struct OurVendorHyperSrcClientConnExtractor;
