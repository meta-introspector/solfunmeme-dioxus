use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/fuzz/corpus/server"]
pub struct OurVendorRustlsFuzzCorpusServerExtractor;
