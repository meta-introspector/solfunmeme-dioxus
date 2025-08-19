use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pki-types/fuzz/corpus"]
pub struct OurVendorPkiTypesFuzzCorpusExtractor;
