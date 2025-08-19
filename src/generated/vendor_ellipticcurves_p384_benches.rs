use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p384/benches"]
pub struct OurVendorEllipticCurvesP384BenchesExtractor;
