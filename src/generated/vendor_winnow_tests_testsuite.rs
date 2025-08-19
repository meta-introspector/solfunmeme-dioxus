use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/winnow/tests/testsuite"]
pub struct OurVendorWinnowTestsTestsuiteExtractor;
