use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/agave/.cargo"]
pub struct OurVendorAgaveCargoExtractor;
