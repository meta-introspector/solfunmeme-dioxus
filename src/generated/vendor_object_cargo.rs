use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/object/.cargo"]
pub struct OurVendorObjectCargoExtractor;
