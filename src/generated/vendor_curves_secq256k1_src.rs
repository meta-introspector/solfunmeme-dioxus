use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secq256k1/src"]
pub struct OurVendorCurvesSecq256k1SrcExtractor;
