use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/server/conn"]
pub struct OurVendorHyperSrcServerConnExtractor;
