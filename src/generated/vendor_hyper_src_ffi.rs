use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/src/ffi"]
pub struct OurVendorHyperSrcFfiExtractor;
