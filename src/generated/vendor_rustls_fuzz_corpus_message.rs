use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/fuzz/corpus/message"]
pub struct OurVendorRustlsFuzzCorpusMessageExtractor;
