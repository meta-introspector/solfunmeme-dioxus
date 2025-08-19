use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/slotmap/fuzz/fuzz_targets"]
pub struct OurVendorSlotmapFuzzFuzzTargetsExtractor;
