use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/curve25519/benches"]
pub struct OurVendorCurvesCurve25519BenchesExtractor;
