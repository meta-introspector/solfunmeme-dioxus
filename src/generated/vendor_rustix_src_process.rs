use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/process"]
pub struct OurVendorRustixSrcProcessExtractor;
