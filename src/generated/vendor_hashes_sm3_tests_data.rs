use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/sm3/tests/data"]
pub struct OurVendorHashesSm3TestsDataExtractor;
