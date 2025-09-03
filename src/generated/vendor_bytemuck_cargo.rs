use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bytemuck/.cargo"]
pub struct OurVendorBytemuckCargoExtractor;
