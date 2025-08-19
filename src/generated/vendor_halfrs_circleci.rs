use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/half-rs/.circleci"]
pub struct OurVendorHalfRsCircleciExtractor;
