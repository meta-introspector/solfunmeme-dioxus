use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring/include"]
pub struct OurVendorRingIncludeExtractor;
