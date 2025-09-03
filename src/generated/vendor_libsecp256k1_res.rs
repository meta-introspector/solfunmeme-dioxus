use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/res"]
pub struct OurVendorLibsecp256k1ResExtractor;
