use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/ed25519/src/curves"]
pub struct OurVendorCurvesEd25519SrcCurvesExtractor;
