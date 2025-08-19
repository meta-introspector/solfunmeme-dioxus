use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/termios"]
pub struct OurVendorRustixTestsTermiosExtractor;
