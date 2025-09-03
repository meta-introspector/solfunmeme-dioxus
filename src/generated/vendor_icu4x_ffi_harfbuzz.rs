use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/harfbuzz"]
pub struct OurVendorIcu4xFfiHarfbuzzExtractor;
