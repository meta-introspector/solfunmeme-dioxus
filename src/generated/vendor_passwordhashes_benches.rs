use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/benches"]
pub struct OurVendorPasswordHashesBenchesExtractor;
