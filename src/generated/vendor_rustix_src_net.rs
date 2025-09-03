use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/net"]
pub struct OurVendorRustixSrcNetExtractor;
