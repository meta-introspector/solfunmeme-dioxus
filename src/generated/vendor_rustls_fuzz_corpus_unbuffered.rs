use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/fuzz/corpus/unbuffered"]
pub struct OurVendorRustlsFuzzCorpusUnbufferedExtractor;
