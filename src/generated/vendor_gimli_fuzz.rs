use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/gimli/fuzz"]
pub struct OurVendorGimliFuzzExtractor;
