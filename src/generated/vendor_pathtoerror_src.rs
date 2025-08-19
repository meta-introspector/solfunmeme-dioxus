use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/path-to-error/src"]
pub struct OurVendorPathToErrorSrcExtractor;
