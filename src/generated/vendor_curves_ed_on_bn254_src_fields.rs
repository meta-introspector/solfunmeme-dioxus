use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/ed_on_bn254/src/fields"]
pub struct OurVendorCurvesEdOnBn254SrcFieldsExtractor;
