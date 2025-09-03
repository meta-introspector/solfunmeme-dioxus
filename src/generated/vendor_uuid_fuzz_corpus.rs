use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/uuid/fuzz/corpus"]
pub struct OurVendorUuidFuzzCorpusExtractor;
