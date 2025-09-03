use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/httparse/fuzz/fuzz_targets"]
pub struct OurVendorHttparseFuzzFuzzTargetsExtractor;
