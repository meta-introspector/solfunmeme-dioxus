use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/subtle/fuzz/fuzzers"]
pub struct OurVendorSubtleFuzzFuzzersExtractor;
