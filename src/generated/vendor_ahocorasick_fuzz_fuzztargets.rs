use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/aho-corasick/fuzz/fuzz-targets"]
pub struct OurVendorAhoCorasickFuzzFuzzTargetsExtractor;
