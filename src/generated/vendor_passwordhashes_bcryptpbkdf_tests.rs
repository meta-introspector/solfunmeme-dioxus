use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/bcrypt-pbkdf/tests"]
pub struct OurVendorPasswordHashesBcryptPbkdfTestsExtractor;
