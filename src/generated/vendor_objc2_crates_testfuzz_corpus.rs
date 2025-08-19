use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/objc2/crates/test-fuzz/corpus"]
pub struct OurVendorObjc2CratesTestFuzzCorpusExtractor;
