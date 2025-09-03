use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/winnow/.cargo"]
pub struct OurVendorWinnowCargoExtractor;
