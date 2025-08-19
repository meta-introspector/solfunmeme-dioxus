use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/tests/mount"]
pub struct OurVendorRustixTestsMountExtractor;
