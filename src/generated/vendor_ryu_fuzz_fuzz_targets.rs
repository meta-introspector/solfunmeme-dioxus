use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ryu/fuzz/fuzz_targets"]
pub struct OurVendorRyuFuzzFuzzTargetsExtractor;
