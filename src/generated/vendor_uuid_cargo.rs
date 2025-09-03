use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/uuid/.cargo"]
pub struct OurVendorUuidCargoExtractor;
