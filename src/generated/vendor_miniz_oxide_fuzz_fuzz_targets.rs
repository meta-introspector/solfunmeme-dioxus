use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/miniz_oxide/fuzz/fuzz_targets"]
pub struct OurVendorMinizOxideFuzzFuzzTargetsExtractor;
