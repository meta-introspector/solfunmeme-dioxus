use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p256/benches"]
pub struct OurVendorEllipticCurvesP256BenchesExtractor;
