use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/scrypt/benches"]
pub struct OurVendorPasswordHashesScryptBenchesExtractor;
