use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/src"]
pub struct OurVendorSynSrcExtractor;
