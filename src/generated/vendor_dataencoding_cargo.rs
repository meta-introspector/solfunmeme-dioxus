use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/data-encoding/.cargo"]
pub struct OurVendorDataEncodingCargoExtractor;
