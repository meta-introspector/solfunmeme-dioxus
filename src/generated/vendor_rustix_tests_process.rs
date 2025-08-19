use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/process"]
pub struct OurVendorRustixTestsProcessExtractor;
