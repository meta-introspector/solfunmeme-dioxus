use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/path"]
pub struct OurVendorRustixTestsPathExtractor;
