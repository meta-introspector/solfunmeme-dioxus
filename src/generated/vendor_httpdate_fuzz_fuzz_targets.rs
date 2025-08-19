use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/httpdate/fuzz/fuzz_targets"]
pub struct OurVendorHttpdateFuzzFuzzTargetsExtractor;
