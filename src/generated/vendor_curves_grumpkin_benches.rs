use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/grumpkin/benches"]
pub struct OurVendorCurvesGrumpkinBenchesExtractor;
