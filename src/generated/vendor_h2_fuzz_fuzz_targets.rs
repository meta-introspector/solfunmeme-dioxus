use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/h2/fuzz/fuzz_targets"]
pub struct OurVendorH2FuzzFuzzTargetsExtractor;
