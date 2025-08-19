use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi-rs/crates"]
pub struct OurVendorWasiRsCratesExtractor;
