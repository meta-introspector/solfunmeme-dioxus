use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/provider/data/plurals/data"]
pub struct OurVendorIcu4xProviderDataPluralsDataExtractor;
