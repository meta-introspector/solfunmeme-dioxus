use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/std/src"]
pub struct OurVendorStdSrcExtractor;
