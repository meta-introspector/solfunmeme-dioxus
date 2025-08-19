use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p384/src/test_vectors/data"]
pub struct OurVendorEllipticCurvesP384SrcTestVectorsDataExtractor;
