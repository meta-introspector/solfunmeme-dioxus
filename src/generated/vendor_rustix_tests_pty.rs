use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/pty"]
pub struct OurVendorRustixTestsPtyExtractor;
