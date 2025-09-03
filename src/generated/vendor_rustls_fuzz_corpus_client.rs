use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/fuzz/corpus/client"]
pub struct OurVendorRustlsFuzzCorpusClientExtractor;
