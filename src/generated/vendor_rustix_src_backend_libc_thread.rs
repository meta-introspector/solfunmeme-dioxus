use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/thread"]
pub struct OurVendorRustixSrcBackendLibcThreadExtractor;
