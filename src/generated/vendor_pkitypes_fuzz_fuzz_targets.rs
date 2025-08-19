use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pki-types/fuzz/fuzz_targets"]
pub struct OurVendorPkiTypesFuzzFuzzTargetsExtractor;
