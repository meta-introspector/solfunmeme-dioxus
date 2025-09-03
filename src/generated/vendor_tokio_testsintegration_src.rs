use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tests-integration/src"]
pub struct OurVendorTokioTestsIntegrationSrcExtractor;
