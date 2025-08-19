use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-normalization/fuzz/fuzz_targets"]
pub struct OurVendorUnicodeNormalizationFuzzFuzzTargetsExtractor;
