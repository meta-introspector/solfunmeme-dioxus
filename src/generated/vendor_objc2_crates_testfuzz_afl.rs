use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/objc2/crates/test-fuzz/afl"]
pub struct OurVendorObjc2CratesTestFuzzAflExtractor;
