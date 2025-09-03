use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/rand"]
pub struct OurVendorRustixSrcRandExtractor;
