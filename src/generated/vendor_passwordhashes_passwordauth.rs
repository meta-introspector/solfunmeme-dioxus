use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/password-auth"]
pub struct OurVendorPasswordHashesPasswordAuthExtractor;
