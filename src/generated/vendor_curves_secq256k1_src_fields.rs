use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secq256k1/src/fields"]
pub struct OurVendorCurvesSecq256k1SrcFieldsExtractor;
