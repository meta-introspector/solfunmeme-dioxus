use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/p256/src/test_vectors/data"]
pub struct OurVendorEllipticCurvesP256SrcTestVectorsDataExtractor;
