use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/gimli/fuzz/fuzz_targets"]
pub struct OurVendorGimliFuzzFuzzTargetsExtractor;
