use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/blake2/src/simd"]
pub struct OurVendorHashesBlake2SrcSimdExtractor;
