use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp256k1/src/curves"]
pub struct OurVendorCurvesSecp256k1SrcCurvesExtractor;
