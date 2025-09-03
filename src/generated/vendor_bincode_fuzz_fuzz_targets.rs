use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bincode/fuzz/fuzz_targets"]
pub struct OurVendorBincodeFuzzFuzzTargetsExtractor;
