use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/k256/src/test_vectors/data"]
pub struct OurVendorEllipticCurvesK256SrcTestVectorsDataExtractor;
