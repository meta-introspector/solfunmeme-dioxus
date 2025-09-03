use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/src/ext"]
pub struct OurVendorWasiSrcExtExtractor;
