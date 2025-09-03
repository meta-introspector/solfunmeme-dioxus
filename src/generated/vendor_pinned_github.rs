use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pinned/.github"]
pub struct OurVendorPinnedGithubExtractor;
