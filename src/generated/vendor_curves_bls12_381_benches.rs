use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bls12_381/benches"]
pub struct OurVendorCurvesBls12381BenchesExtractor;
