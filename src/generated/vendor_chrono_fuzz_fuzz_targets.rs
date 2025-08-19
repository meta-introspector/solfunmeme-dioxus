use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/chrono/fuzz/fuzz_targets"]
pub struct OurVendorChronoFuzzFuzzTargetsExtractor;
