use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/yescrypt/tests"]
pub struct OurVendorPasswordHashesYescryptTestsExtractor;
