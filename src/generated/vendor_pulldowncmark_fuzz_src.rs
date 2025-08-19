use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pulldown-cmark/fuzz/src"]
pub struct OurVendorPulldownCmarkFuzzSrcExtractor;
