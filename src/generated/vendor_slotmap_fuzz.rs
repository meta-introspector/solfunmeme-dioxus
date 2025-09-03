use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/slotmap/fuzz"]
pub struct OurVendorSlotmapFuzzExtractor;
