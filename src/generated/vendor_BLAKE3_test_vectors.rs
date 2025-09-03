use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/BLAKE3/test_vectors"]
pub struct OurVendorBLAKE3TestVectorsExtractor;
