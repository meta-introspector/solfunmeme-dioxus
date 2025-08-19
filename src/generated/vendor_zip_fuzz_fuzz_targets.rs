use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/zip/fuzz/fuzz_targets"]
pub struct OurVendorZipFuzzFuzzTargetsExtractor;
