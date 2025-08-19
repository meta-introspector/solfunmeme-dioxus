use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/ci"]
pub struct OurVendorRustixCiExtractor;
