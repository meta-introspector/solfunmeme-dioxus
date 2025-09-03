use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/markdown-rs/fuzz/fuzz_targets"]
pub struct OurVendorMarkdownRsFuzzFuzzTargetsExtractor;
