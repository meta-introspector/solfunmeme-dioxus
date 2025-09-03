use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tests-integration/tests"]
pub struct OurVendorTokioTestsIntegrationTestsExtractor;
