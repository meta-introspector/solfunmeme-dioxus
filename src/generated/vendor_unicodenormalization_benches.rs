use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/unicode-normalization/benches"]
pub struct OurVendorUnicodeNormalizationBenchesExtractor;
