use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/reqwasm/src"]
pub struct OurVendorReqwasmSrcExtractor;
