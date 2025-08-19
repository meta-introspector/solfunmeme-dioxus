use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/password-hashes/.github"]
pub struct OurVendorPasswordHashesGithubExtractor;
