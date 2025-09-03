use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-normalization/tests/data"]
pub struct OurVendorUnicodeNormalizationTestsDataExtractor;
