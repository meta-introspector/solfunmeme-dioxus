use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/proto"]
pub struct OurVendorHyperSrcProtoExtractor;
