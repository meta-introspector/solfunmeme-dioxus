use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/core"]
pub struct OurVendorLibsecp256k1CoreExtractor;
