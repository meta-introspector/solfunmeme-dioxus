use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/mio/src"]
pub struct OurVendorMioSrcExtractor;
