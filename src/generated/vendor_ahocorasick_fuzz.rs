use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/aho-corasick/fuzz"]
pub struct OurVendorAhoCorasickFuzzExtractor;
