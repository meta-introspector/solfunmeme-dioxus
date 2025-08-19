use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/tests/regression"]
pub struct OurVendorSynTestsRegressionExtractor;
