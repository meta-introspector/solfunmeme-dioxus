use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/io_uring"]
pub struct OurVendorRustixSrcIoUringExtractor;
