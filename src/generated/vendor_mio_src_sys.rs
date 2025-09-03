use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/mio/src/sys"]
pub struct OurVendorMioSrcSysExtractor;
