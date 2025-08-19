use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/components/collator/fuzz/fuzz_targets"]
pub struct OurVendorIcu4xComponentsCollatorFuzzFuzzTargetsExtractor;
