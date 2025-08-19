use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-segmentation/fuzz/fuzz_targets"]
pub struct OurVendorUnicodeSegmentationFuzzFuzzTargetsExtractor;
