use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bn254/src/curves"]
pub struct OurVendorCurvesBn254SrcCurvesExtractor;
