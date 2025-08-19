use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rngs/rand_xorshift/src"]
pub struct OurVendorRngsRandXorshiftSrcExtractor;
