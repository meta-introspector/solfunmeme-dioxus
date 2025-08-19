use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/fs"]
pub struct OurVendorRustixTestsFsExtractor;
