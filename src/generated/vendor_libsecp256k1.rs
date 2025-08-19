use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1"]
pub struct OurVendorLibsecp256k1Extractor;
