use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/.cargo"]
pub struct OurVendorSignaturesCargoExtractor;
