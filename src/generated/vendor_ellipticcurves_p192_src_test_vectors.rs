use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p192/src/test_vectors"]
pub struct OurVendorEllipticCurvesP192SrcTestVectorsExtractor;
