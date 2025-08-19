use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/cryptocorrosion/.cargo"]
pub struct OurVendorCryptocorrosionCargoExtractor;
