use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/provider/data/locale/data"]
pub struct OurVendorIcu4xProviderDataLocaleDataExtractor;
