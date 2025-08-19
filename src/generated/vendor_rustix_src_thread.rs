use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/thread"]
pub struct OurVendorRustixSrcThreadExtractor;
