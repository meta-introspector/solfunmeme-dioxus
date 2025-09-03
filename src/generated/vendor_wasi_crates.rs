use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/crates"]
pub struct OurVendorWasiCratesExtractor;
