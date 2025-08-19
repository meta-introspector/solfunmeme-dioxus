use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/bign256/src/test_vectors"]
pub struct OurVendorEllipticCurvesBign256SrcTestVectorsExtractor;
