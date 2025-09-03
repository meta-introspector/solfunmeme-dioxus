use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/fs"]
pub struct OurVendorRustixSrcFsExtractor;
