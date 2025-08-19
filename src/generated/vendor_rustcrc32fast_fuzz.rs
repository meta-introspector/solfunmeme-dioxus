use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-crc32fast/fuzz"]
pub struct OurVendorRustCrc32fastFuzzExtractor;
