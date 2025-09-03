use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/libsecp256k1/gen/ecmult/src"]
pub struct OurVendorLibsecp256k1GenEcmultSrcExtractor;
