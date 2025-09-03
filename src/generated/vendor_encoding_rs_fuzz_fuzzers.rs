use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/encoding_rs/fuzz/fuzzers"]
pub struct OurVendorEncodingRsFuzzFuzzersExtractor;
