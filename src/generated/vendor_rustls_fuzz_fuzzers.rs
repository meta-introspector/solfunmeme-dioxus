use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/fuzz/fuzzers"]
pub struct OurVendorRustlsFuzzFuzzersExtractor;
