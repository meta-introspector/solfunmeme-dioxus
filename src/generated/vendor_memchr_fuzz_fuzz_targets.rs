use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/memchr/fuzz/fuzz_targets"]
pub struct OurVendorMemchrFuzzFuzzTargetsExtractor;
