use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests"]
pub struct OurVendorRustixTestsExtractor;
