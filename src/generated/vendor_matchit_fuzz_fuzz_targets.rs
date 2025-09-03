use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/matchit/fuzz/fuzz_targets"]
pub struct OurVendorMatchitFuzzFuzzTargetsExtractor;
