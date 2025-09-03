use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hex-conservative/fuzz/fuzz_targets"]
pub struct OurVendorHexConservativeFuzzFuzzTargetsExtractor;
