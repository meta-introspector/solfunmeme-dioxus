use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/ci"]
pub struct OurVendorWasiCiExtractor;
