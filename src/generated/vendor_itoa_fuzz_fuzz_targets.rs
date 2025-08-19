use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/itoa/fuzz/fuzz_targets"]
pub struct OurVendorItoaFuzzFuzzTargetsExtractor;
