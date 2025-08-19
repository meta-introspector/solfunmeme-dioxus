use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/serde/test_suite/no_std/src"]
pub struct OurVendorSerdeTestSuiteNoStdSrcExtractor;
