use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/instant/.circleci"]
pub struct OurVendorInstantCircleciExtractor;
