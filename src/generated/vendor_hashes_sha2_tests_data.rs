use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/sha2/tests/data"]
pub struct OurVendorHashesSha2TestsDataExtractor;
