use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/scrypt/tests"]
pub struct OurVendorPasswordHashesScryptTestsExtractor;
