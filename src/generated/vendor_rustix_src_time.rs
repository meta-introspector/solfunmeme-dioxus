use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/time"]
pub struct OurVendorRustixSrcTimeExtractor;
