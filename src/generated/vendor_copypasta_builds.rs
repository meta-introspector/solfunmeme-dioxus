use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/copypasta/.builds"]
pub struct OurVendorCopypastaBuildsExtractor;
