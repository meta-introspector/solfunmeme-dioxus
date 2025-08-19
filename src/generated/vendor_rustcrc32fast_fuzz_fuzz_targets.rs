use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-crc32fast/fuzz/fuzz_targets"]
pub struct OurVendorRustCrc32fastFuzzFuzzTargetsExtractor;
