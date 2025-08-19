use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bumpalo/src"]
pub struct OurVendorBumpaloSrcExtractor;
