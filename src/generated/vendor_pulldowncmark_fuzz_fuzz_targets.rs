use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pulldown-cmark/fuzz/fuzz_targets"]
pub struct OurVendorPulldownCmarkFuzzFuzzTargetsExtractor;
