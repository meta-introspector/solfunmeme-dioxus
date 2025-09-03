use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/serde/test_suite/tests"]
pub struct OurVendorSerdeTestSuiteTestsExtractor;
