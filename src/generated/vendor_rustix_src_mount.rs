use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/mount"]
pub struct OurVendorRustixSrcMountExtractor;
