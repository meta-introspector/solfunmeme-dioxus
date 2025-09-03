use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/sha3/tests/data"]
pub struct OurVendorHashesSha3TestsDataExtractor;
