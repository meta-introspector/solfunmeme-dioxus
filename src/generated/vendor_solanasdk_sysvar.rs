use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/sysvar"]
pub struct OurVendorSolanaSdkSysvarExtractor;
