use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-base64/fuzz/fuzzers"]
pub struct OurVendorRustBase64FuzzFuzzersExtractor;
