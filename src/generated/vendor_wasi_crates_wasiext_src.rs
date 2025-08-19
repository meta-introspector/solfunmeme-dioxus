use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/crates/wasi-ext/src"]
pub struct OurVendorWasiCratesWasiExtSrcExtractor;
