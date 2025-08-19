use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/wit"]
pub struct OurVendorWasiWitExtractor;
