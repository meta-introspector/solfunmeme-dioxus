use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp256k1/src/fields"]
pub struct OurVendorCurvesSecp256k1SrcFieldsExtractor;
