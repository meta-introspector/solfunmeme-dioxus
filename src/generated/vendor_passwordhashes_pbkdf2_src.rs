use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/pbkdf2/src"]
pub struct OurVendorPasswordHashesPbkdf2SrcExtractor;
