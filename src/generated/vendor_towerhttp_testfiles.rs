use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tower-http/test-files"]
pub struct OurVendorTowerHttpTestFilesExtractor;
