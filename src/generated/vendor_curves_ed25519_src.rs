use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/ed25519/src"]
pub struct OurVendorCurvesEd25519SrcExtractor;
