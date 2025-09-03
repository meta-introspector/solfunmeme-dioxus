use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rand/src/rngs"]
pub struct OurVendorRandSrcRngsExtractor;
