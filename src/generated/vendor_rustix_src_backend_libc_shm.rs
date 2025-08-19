use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/shm"]
pub struct OurVendorRustixSrcBackendLibcShmExtractor;
