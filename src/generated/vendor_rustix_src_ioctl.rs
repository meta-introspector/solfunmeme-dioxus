use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/ioctl"]
pub struct OurVendorRustixSrcIoctlExtractor;
