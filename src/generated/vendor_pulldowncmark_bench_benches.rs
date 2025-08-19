use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pulldown-cmark/bench/benches"]
pub struct OurVendorPulldownCmarkBenchBenchesExtractor;
