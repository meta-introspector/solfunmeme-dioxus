use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bls12_381/src/fields"]
pub struct OurVendorCurvesBls12381SrcFieldsExtractor;
