use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/fs"]
pub struct OurVendorRustixSrcBackendLibcFsExtractor;
