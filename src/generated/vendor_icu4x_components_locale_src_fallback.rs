use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/components/locale/src/fallback"]
pub struct OurVendorIcu4xComponentsLocaleSrcFallbackExtractor;
