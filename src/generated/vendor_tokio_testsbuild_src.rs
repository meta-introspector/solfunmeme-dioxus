use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/tests-build/src"]
pub struct OurVendorTokioTestsBuildSrcExtractor;
