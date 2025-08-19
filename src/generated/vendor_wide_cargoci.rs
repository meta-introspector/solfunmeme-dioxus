use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wide/.cargo-ci"]
pub struct OurVendorWideCargoCiExtractor;
