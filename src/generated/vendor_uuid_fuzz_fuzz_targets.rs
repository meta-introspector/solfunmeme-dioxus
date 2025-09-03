use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/uuid/fuzz/fuzz_targets"]
pub struct OurVendorUuidFuzzFuzzTargetsExtractor;
