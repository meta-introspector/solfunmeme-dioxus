use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/path"]
pub struct OurVendorRustixSrcPathExtractor;
