use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi-rs/src"]
pub struct OurVendorWasiRsSrcExtractor;
