use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/backend/libc/termios"]
pub struct OurVendorRustixSrcBackendLibcTermiosExtractor;
