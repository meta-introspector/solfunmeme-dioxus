use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p521/src/test_vectors/data"]
pub struct OurVendorEllipticCurvesP521SrcTestVectorsDataExtractor;
