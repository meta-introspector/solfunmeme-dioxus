use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/io_uring"]
pub struct OurVendorRustixSrcBackendLibcIoUringExtractor;
