use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/uriparse-rs/fuzz/fuzz_targets"]
pub struct OurVendorUriparseRsFuzzFuzzTargetsExtractor;
