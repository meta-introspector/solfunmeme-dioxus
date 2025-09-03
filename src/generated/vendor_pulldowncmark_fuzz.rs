use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pulldown-cmark/fuzz"]
pub struct OurVendorPulldownCmarkFuzzExtractor;
