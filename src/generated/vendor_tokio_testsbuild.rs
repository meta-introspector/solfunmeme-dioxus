use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tests-build"]
pub struct OurVendorTokioTestsBuildExtractor;
