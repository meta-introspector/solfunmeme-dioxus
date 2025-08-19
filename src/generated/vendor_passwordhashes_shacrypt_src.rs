use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/sha-crypt/src"]
pub struct OurVendorPasswordHashesShaCryptSrcExtractor;
