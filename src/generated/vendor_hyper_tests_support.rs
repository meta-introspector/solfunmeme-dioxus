use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/hyper/tests/support"]
pub struct OurVendorHyperTestsSupportExtractor;
