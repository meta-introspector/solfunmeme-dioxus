use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/system"]
pub struct OurVendorRustixSrcBackendLibcSystemExtractor;
