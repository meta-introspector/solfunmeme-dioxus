use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp256r1/src"]
pub struct OurVendorCurvesSecp256r1SrcExtractor;
