use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp256r1/src/constraints"]
pub struct OurVendorCurvesSecp256r1SrcConstraintsExtractor;
