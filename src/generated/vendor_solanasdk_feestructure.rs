use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/fee-structure"]
pub struct OurVendorSolanaSdkFeeStructureExtractor;
