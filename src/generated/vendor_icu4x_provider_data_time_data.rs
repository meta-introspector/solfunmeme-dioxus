use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/provider/data/time/data"]
pub struct OurVendorIcu4xProviderDataTimeDataExtractor;
