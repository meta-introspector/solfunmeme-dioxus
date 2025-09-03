use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/process"]
pub struct OurVendorRustixSrcBackendLibcProcessExtractor;
