use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/reqwest/src"]
pub struct OurVendorReqwestSrcExtractor;
