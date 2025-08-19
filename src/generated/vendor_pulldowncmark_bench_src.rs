use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pulldown-cmark/bench/src"]
pub struct OurVendorPulldownCmarkBenchSrcExtractor;
