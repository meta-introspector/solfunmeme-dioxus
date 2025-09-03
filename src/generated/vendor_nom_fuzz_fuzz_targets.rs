use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nom/fuzz/fuzz_targets"]
pub struct OurVendorNomFuzzFuzzTargetsExtractor;
