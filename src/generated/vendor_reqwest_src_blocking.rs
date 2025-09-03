use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/reqwest/src/blocking"]
pub struct OurVendorReqwestSrcBlockingExtractor;
