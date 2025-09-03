use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/termios"]
pub struct OurVendorRustixSrcTermiosExtractor;
