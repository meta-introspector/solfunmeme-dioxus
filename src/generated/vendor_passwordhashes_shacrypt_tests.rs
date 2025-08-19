use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/sha-crypt/tests"]
pub struct OurVendorPasswordHashesShaCryptTestsExtractor;
