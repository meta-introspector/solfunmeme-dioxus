use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/crossbeam/ci"]
pub struct OurVendorCrossbeamCiExtractor;
