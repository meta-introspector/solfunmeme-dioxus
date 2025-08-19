use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/core/src"]
pub struct OurVendorLibsecp256k1CoreSrcExtractor;
