use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/http/fuzz/src"]
pub struct OurVendorHttpFuzzSrcExtractor;
