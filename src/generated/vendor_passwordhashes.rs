use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes"]
pub struct OurVendorPasswordHashesExtractor;
