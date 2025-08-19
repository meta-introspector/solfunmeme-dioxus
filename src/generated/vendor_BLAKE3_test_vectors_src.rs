use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/BLAKE3/test_vectors/src"]
pub struct OurVendorBLAKE3TestVectorsSrcExtractor;
