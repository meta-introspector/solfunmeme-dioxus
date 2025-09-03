use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/objc2/crates/test-fuzz/fuzz_targets"]
pub struct OurVendorObjc2CratesTestFuzzFuzzTargetsExtractor;
