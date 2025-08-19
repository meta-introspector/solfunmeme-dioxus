use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/provider/data/plurals"]
pub struct OurVendorIcu4xProviderDataPluralsExtractor;
