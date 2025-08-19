use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp256r1/src/curves"]
pub struct OurVendorCurvesSecp256r1SrcCurvesExtractor;
