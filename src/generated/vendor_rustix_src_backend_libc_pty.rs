use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/pty"]
pub struct OurVendorRustixSrcBackendLibcPtyExtractor;
