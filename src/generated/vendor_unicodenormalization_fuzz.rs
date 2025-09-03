use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-normalization/fuzz"]
pub struct OurVendorUnicodeNormalizationFuzzExtractor;
