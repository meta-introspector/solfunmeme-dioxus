use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bn254/src"]
pub struct OurVendorCurvesBn254SrcExtractor;
