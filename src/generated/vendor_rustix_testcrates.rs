use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/test-crates"]
pub struct OurVendorRustixTestCratesExtractor;
