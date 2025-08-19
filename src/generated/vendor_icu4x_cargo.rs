use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/icu4x/.cargo"]
pub struct OurVendorIcu4xCargoExtractor;
