use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper-tls/src"]
pub struct OurVendorHyperTlsSrcExtractor;
