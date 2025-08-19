use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/stdio"]
pub struct OurVendorRustixTestsStdioExtractor;
