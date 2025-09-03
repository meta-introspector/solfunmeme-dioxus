use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/provider/data/locale"]
pub struct OurVendorIcu4xProviderDataLocaleExtractor;
