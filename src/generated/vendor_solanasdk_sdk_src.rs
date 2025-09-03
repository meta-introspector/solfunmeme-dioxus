use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/sdk/src"]
pub struct OurVendorSolanaSdkSdkSrcExtractor;
