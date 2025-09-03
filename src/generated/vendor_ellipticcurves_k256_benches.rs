use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/k256/benches"]
pub struct OurVendorEllipticCurvesK256BenchesExtractor;
