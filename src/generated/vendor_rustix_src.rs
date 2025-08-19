use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src"]
pub struct OurVendorRustixSrcExtractor;
