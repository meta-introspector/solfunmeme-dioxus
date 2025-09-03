use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/bls12_377/benches"]
pub struct OurVendorCurvesBls12377BenchesExtractor;
