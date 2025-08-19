use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/crates/wasi-ext"]
pub struct OurVendorWasiCratesWasiExtExtractor;
