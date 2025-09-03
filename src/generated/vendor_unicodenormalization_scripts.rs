use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-normalization/scripts"]
pub struct OurVendorUnicodeNormalizationScriptsExtractor;
