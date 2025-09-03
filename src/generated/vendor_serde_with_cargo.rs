use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/serde_with/.cargo"]
pub struct OurVendorSerdeWithCargoExtractor;
