use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/getrandom/.cargo"]
pub struct OurVendorGetrandomCargoExtractor;
