use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/fuzz"]
pub struct OurVendorPasswordHashesFuzzExtractor;
