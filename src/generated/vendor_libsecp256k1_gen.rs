use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/gen"]
pub struct OurVendorLibsecp256k1GenExtractor;
