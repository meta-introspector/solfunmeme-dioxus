use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rand/rand_core/src"]
pub struct OurVendorRandRandCoreSrcExtractor;
