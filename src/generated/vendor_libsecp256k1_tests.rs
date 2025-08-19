use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/tests"]
pub struct OurVendorLibsecp256k1TestsExtractor;
