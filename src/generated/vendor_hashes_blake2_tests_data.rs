use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/blake2/tests/data"]
pub struct OurVendorHashesBlake2TestsDataExtractor;
