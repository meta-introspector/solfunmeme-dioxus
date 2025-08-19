use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/toml/.github"]
pub struct OurVendorTomlGithubExtractor;
