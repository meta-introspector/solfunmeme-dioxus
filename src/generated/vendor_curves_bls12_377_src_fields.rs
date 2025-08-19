use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bls12_377/src/fields"]
pub struct OurVendorCurvesBls12377SrcFieldsExtractor;
