use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/curve25519/src/fields"]
pub struct OurVendorCurvesCurve25519SrcFieldsExtractor;
