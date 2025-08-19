use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp384r1/src"]
pub struct OurVendorCurvesSecp384r1SrcExtractor;
