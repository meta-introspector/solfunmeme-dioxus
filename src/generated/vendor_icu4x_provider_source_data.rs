use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/provider/source/data"]
pub struct OurVendorIcu4xProviderSourceDataExtractor;
