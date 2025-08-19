use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/src"]
pub struct OurVendorLibsecp256k1SrcExtractor;
