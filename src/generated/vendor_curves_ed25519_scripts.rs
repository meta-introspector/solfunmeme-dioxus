use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/ed25519/scripts"]
pub struct OurVendorCurvesEd25519ScriptsExtractor;
