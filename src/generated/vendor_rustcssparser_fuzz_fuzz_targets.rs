use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rust-cssparser/fuzz/fuzz_targets"]
pub struct OurVendorRustCssparserFuzzFuzzTargetsExtractor;
