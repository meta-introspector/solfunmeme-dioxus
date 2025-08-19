use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bls12_381/src/curves"]
pub struct OurVendorCurvesBls12381SrcCurvesExtractor;
