use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-segmentation/fuzz"]
pub struct OurVendorUnicodeSegmentationFuzzExtractor;
