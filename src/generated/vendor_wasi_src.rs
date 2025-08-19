use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/src"]
pub struct OurVendorWasiSrcExtractor;
