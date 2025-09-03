use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/argon2/tests"]
pub struct OurVendorPasswordHashesArgon2TestsExtractor;
