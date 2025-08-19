use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bincode/fuzz"]
pub struct OurVendorBincodeFuzzExtractor;
