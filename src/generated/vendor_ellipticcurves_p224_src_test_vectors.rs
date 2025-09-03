use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p224/src/test_vectors"]
pub struct OurVendorEllipticCurvesP224SrcTestVectorsExtractor;
