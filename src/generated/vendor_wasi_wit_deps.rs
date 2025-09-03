use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wasi/wit/deps"]
pub struct OurVendorWasiWitDepsExtractor;
