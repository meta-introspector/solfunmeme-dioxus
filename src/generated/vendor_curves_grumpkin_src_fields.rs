use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/grumpkin/src/fields"]
pub struct OurVendorCurvesGrumpkinSrcFieldsExtractor;
