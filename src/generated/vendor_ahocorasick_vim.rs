use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/aho-corasick/.vim"]
pub struct OurVendorAhoCorasickVimExtractor;
