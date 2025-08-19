use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/fuzz/fuzz_targets"]
pub struct OurVendorSynFuzzFuzzTargetsExtractor;
