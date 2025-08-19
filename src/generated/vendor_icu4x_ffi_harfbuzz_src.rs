use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/ffi/harfbuzz/src"]
pub struct OurVendorIcu4xFfiHarfbuzzSrcExtractor;
