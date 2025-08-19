use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/winnow/fuzz/fuzz_targets"]
pub struct OurVendorWinnowFuzzFuzzTargetsExtractor;
