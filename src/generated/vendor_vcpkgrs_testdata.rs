use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/vcpkg-rs/test-data"]
pub struct OurVendorVcpkgRsTestDataExtractor;
