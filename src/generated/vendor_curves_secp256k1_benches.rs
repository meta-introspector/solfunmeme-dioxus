use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curves/secp256k1/benches"]
pub struct OurVendorCurvesSecp256k1BenchesExtractor;
