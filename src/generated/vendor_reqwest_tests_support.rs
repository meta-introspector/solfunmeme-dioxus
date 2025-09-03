use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/reqwest/tests/support"]
pub struct OurVendorReqwestTestsSupportExtractor;
