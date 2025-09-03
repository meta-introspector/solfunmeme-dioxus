use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/commitment-config"]
pub struct OurVendorSolanaSdkCommitmentConfigExtractor;
