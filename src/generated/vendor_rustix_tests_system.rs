use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/system"]
pub struct OurVendorRustixTestsSystemExtractor;
