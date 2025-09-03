use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/yescrypt/src"]
pub struct OurVendorPasswordHashesYescryptSrcExtractor;
