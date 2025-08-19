use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/components/normalizer/fuzz/fuzz_targets"]
pub struct OurVendorIcu4xComponentsNormalizerFuzzFuzzTargetsExtractor;
