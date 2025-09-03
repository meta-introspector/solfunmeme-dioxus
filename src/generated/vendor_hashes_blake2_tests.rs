use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/blake2/tests"]
pub struct OurVendorHashesBlake2TestsExtractor;
