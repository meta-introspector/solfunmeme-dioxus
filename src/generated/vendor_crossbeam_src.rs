use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/crossbeam/src"]
pub struct OurVendorCrossbeamSrcExtractor;
