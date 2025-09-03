use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/scrypt"]
pub struct OurVendorPasswordHashesScryptExtractor;
