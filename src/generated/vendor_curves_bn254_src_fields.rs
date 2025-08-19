use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bn254/src/fields"]
pub struct OurVendorCurvesBn254SrcFieldsExtractor;
