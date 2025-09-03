use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/utf8_iter/fuzz/fuzz_targets"]
pub struct OurVendorUtf8IterFuzzFuzzTargetsExtractor;
