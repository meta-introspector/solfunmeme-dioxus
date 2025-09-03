use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/yescrypt"]
pub struct OurVendorPasswordHashesYescryptExtractor;
