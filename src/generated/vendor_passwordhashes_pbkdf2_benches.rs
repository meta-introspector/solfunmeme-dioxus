use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/pbkdf2/benches"]
pub struct OurVendorPasswordHashesPbkdf2BenchesExtractor;
