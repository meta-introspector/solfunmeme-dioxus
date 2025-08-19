use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper-rustls/src"]
pub struct OurVendorHyperRustlsSrcExtractor;
