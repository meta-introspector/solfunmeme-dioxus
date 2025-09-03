use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/scrypt/src"]
pub struct OurVendorPasswordHashesScryptSrcExtractor;
