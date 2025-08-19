use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-url/url/fuzz/fuzz_targets"]
pub struct OurVendorRustUrlUrlFuzzFuzzTargetsExtractor;
