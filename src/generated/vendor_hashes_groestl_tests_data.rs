use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hashes/groestl/tests/data"]
pub struct OurVendorHashesGroestlTestsDataExtractor;
