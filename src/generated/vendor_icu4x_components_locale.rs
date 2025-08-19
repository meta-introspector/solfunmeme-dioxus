use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/components/locale"]
pub struct OurVendorIcu4xComponentsLocaleExtractor;
