use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/json/fuzz/fuzz_targets"]
pub struct OurVendorJsonFuzzFuzzTargetsExtractor;
