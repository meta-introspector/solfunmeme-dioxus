use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p521/benches"]
pub struct OurVendorEllipticCurvesP521BenchesExtractor;
