use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/pipe"]
pub struct OurVendorRustixSrcBackendLibcPipeExtractor;
