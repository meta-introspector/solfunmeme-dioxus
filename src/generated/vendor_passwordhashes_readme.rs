use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/.readme"]
pub struct OurVendorPasswordHashesReadmeExtractor;
