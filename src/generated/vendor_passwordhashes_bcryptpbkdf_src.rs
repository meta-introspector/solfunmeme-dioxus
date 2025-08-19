use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/bcrypt-pbkdf/src"]
pub struct OurVendorPasswordHashesBcryptPbkdfSrcExtractor;
