use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/formats/.cargo"]
pub struct OurVendorFormatsCargoExtractor;
