use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/untrusted/.github"]
pub struct OurVendorUntrustedGithubExtractor;
