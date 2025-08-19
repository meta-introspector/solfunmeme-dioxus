use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/argon2/src"]
pub struct OurVendorPasswordHashesArgon2SrcExtractor;
