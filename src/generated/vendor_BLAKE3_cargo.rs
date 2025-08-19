use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/BLAKE3/.cargo"]
pub struct OurVendorBLAKE3CargoExtractor;
