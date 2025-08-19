use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bls12_381/src"]
pub struct OurVendorCurvesBls12381SrcExtractor;
